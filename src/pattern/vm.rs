//! Tiny Thompson-style VM for walking Gordian Envelope trees.
//!
//! The VM runs byte-code produced by `Pattern::compile` (implemented later).

use bc_components::DigestProvider;
use bc_envelope::prelude::*;

use super::{Matcher, Path, Pattern};
use crate::{Quantifier, Reluctance};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    Subject,
    Assertion,
    Predicate,
    Object,
    Wrapped,
}

impl Axis {
    /// Return `(child, EdgeType)` pairs reachable from `env` via this axis.
    pub fn children(&self, env: &Envelope) -> Vec<(Envelope, EdgeType)> {
        // println!("Axis::children called with axis: {:?}", self);
        // println!("Case: {:?}", env.case());
        // println!("Envelope: {}", env.format_flat());
        match (self, env.case()) {
            (Axis::Subject, EnvelopeCase::Node { subject, .. }) => {
                vec![(subject.clone(), EdgeType::Subject)]
            }
            (Axis::Assertion, EnvelopeCase::Node { assertions, .. }) => {
                assertions
                    .iter()
                    .cloned()
                    .map(|a| (a, EdgeType::Assertion))
                    .collect()
            }
            (Axis::Predicate, EnvelopeCase::Assertion(a)) => {
                vec![(a.predicate().clone(), EdgeType::Predicate)]
            }
            (Axis::Object, EnvelopeCase::Assertion(a)) => {
                vec![(a.object().clone(), EdgeType::Object)]
            }
            (Axis::Wrapped, EnvelopeCase::Node { subject, .. }) => {
                if subject.is_wrapped() {
                    vec![(subject.try_unwrap().unwrap(), EdgeType::Content)]
                } else {
                    vec![]
                }
            }
            (Axis::Wrapped, EnvelopeCase::Wrapped { envelope, .. }) => {
                vec![(envelope.clone(), EdgeType::Content)]
            }
            _ => Vec::new(),
        }
    }
}

/// Bytecode instructions for the pattern VM.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Instr {
    /// Match predicate: `literals[idx].matches(env)`
    MatchPredicate(usize),
    /// Match structure: use `literals[idx].paths(env)` for structure patterns
    MatchStructure(usize),
    /// ε-split: fork execution to `a` and `b`
    Split { a: usize, b: usize },
    /// Unconditional jump to instruction at index
    Jump(usize),
    /// Descend to children via axis, one thread per child
    PushAxis(Axis),
    /// Pop one envelope from the path
    Pop,
    /// Emit current path
    Save,
    /// Final accept, emit current path and halt thread
    Accept,
    /// Recursively search for pattern at `pat_idx` and propagate captures
    Search {
        pat_idx: usize,
        capture_map: Vec<(String, usize)>,
    },
    /// Save current path and start new traversal from last envelope
    ExtendTraversal,
    /// Combine saved path with current path for final result
    CombineTraversal,
    /// Navigate to subject of current envelope
    NavigateSubject,
    /// Match only if pattern at `pat_idx` does not match
    NotMatch { pat_idx: usize },
    /// Repeat a sub pattern according to range and greediness
    Repeat {
        pat_idx: usize,
        quantifier: Quantifier,
    },
    /// Mark the start of a capture group
    CaptureStart(usize),
    /// Mark the end of a capture group
    CaptureEnd(usize),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub code: Vec<Instr>,
    pub literals: Vec<Pattern>,
    pub capture_names: Vec<String>,
}

/// Internal back-tracking state.
#[derive(Clone)]
struct Thread {
    pc: usize,
    env: Envelope,
    path: Path,
    /// Stack of saved paths for nested traversal patterns
    saved_paths: Vec<Path>,
    captures: Vec<Vec<Path>>,
    capture_stack: Vec<Vec<usize>>,
    seen: std::collections::HashSet<Vec<bc_components::Digest>>,
}

/// Match atomic patterns without recursion into the VM.
///
/// This function handles only the patterns that are safe to use in
/// MatchPredicate instructions - Leaf, Structure, Any, and None patterns. Meta
/// patterns should never be passed to this function as they need to be compiled
/// to bytecode.
///
/// For SearchPattern, we provide a temporary fallback that uses the old
/// Match atomic patterns and return both paths and captures.
///
/// This function handles patterns that can produce captures (like CBOR patterns
/// with named groups). This is the primary function used by the VM for pattern
/// execution with capture support.
#[allow(clippy::panic)]
pub(crate) fn atomic_paths_with_captures(
    p: &crate::pattern::Pattern,
    env: &Envelope,
) -> (Vec<Path>, std::collections::HashMap<String, Vec<Path>>) {
    use crate::pattern::Pattern::*;
    match p {
        Leaf(l) => l.paths_with_captures(env),
        Structure(s) => s.paths_with_captures(env),
        Meta(meta) => match meta {
            crate::pattern::meta::MetaPattern::Any(a) => {
                a.paths_with_captures(env)
            }
            crate::pattern::meta::MetaPattern::Search(_) => {
                panic!(
                    "SearchPattern should be compiled to Search instruction, not MatchPredicate"
                );
            }
            _ => panic!(
                "non-atomic meta pattern used in MatchPredicate: {:?}",
                meta
            ),
        },
    }
}

fn repeat_paths(
    pat: &Pattern,
    env: &Envelope,
    path: &Path,
    quantifier: Quantifier,
) -> Vec<(Envelope, Path)> {
    // Build states for all possible repetition counts
    let mut states: Vec<Vec<(Envelope, Path)>> =
        vec![vec![(env.clone(), path.clone())]];
    let bound = quantifier.max().unwrap_or(usize::MAX);

    // Try matching the pattern repeatedly
    for _ in 0..bound {
        let mut next = Vec::new();
        for (e, pth) in states.last().unwrap().iter() {
            for sub_path in pat.paths(e) {
                if let Some(last) = sub_path.last() {
                    if last.digest() == e.digest() {
                        continue; // Avoid infinite loops
                    }
                    let mut combined = pth.clone();
                    if sub_path.first() == Some(e) {
                        combined.extend(sub_path.iter().skip(1).cloned());
                    } else {
                        combined.extend(sub_path.iter().cloned());
                    }
                    next.push((last.clone(), combined));
                }
            }
        }
        if next.is_empty() {
            break; // No more matches possible
        }
        states.push(next);
    }

    // Zero repetition case
    let has_zero_rep = quantifier.min() == 0;
    let zero_rep_result = if has_zero_rep {
        vec![(env.clone(), path.clone())]
    } else {
        vec![]
    };

    // Calculate maximum allowed repetitions
    let max_possible = states.len() - 1;
    let max_allowed = bound.min(max_possible);

    // Check if we can satisfy the minimum repetition requirement
    if max_allowed < quantifier.min() && quantifier.min() > 0 {
        return Vec::new();
    }

    // Calculate the range of repetition counts based on min and max
    // Ensure we don't include zero here - it's handled separately
    let min_count = if quantifier.min() == 0 {
        1
    } else {
        quantifier.min()
    };
    let max_count = if max_allowed < min_count {
        return zero_rep_result;
    } else {
        max_allowed
    };

    let count_range = min_count..=max_count;

    // Generate list of counts to try based on reluctance
    let counts: Vec<usize> = match quantifier.reluctance() {
        Reluctance::Greedy => count_range.rev().collect(),
        Reluctance::Lazy => count_range.collect(),
        Reluctance::Possessive => {
            if max_count >= min_count {
                vec![max_count]
            } else {
                vec![]
            }
        }
    };

    // Collect results based on the counts determined above
    let mut out = Vec::new();

    // For greedy repetition, try higher counts first
    if matches!(quantifier.reluctance(), Reluctance::Greedy) {
        // Include results from counts determined by reluctance
        for c in counts {
            if let Some(list) = states.get(c) {
                out.extend(list.clone());
            }
        }

        // For greedy matching, add zero repetition case at the end if
        // applicable
        if has_zero_rep && out.is_empty() {
            out.push((env.clone(), path.clone()));
        }
    } else {
        // For lazy/possessive, include zero repetition first if applicable
        if has_zero_rep {
            out.push((env.clone(), path.clone()));
        }

        // Then include results from counts determined by reluctance
        for c in counts {
            if let Some(list) = states.get(c) {
                out.extend(list.clone());
            }
        }
    }

    out
}

/// Execute `prog` starting at `root`.  Every time `SAVE` or `ACCEPT` executes,
/// current `path` is pushed into result.
/// Execute a single thread until it halts. Returns true if any paths were
/// produced.
fn run_thread(
    prog: &Program,
    start: Thread,
    out: &mut Vec<(Path, Vec<Vec<Path>>)>,
) -> bool {
    use Instr::*;
    let mut produced = false;
    let mut stack = vec![start];

    while let Some(mut th) = stack.pop() {
        loop {
            match prog.code[th.pc] {
                MatchPredicate(idx) => {
                    let (paths, pattern_captures) = atomic_paths_with_captures(
                        &prog.literals[idx],
                        &th.env,
                    );
                    if paths.is_empty() {
                        break;
                    }

                    th.pc += 1; // Advance to next instruction

                    // Handle multiple paths from atomic patterns (e.g., CBOR
                    // patterns) Process paths in reverse
                    // order for spawning to preserve original order
                    // since stack is LIFO
                    let paths_vec: Vec<_> = paths.into_iter().collect();

                    // Distribute captures fairly across paths
                    // For each capture group, we need to associate captures
                    // with their corresponding paths
                    let mut distributed_captures: Vec<
                        std::collections::HashMap<String, Vec<Path>>,
                    > = vec![std::collections::HashMap::new(); paths_vec.len()];

                    for (name, capture_paths) in pattern_captures {
                        // If we have the same number of paths as captures,
                        // distribute 1:1
                        if capture_paths.len() == paths_vec.len() {
                            for (path_idx, capture_path) in
                                capture_paths.into_iter().enumerate()
                            {
                                if path_idx < distributed_captures.len() {
                                    distributed_captures[path_idx]
                                        .entry(name.clone())
                                        .or_default()
                                        .push(capture_path);
                                }
                            }
                        } else {
                            // Fallback: give all captures to the first path
                            // (this maintains backwards compatibility)
                            if !distributed_captures.is_empty() {
                                distributed_captures[0]
                                    .entry(name)
                                    .or_default()
                                    .extend(capture_paths);
                            }
                        }
                    }

                    for (i, path) in paths_vec.iter().enumerate() {
                        if i == 0 {
                            // Use the first path for the current thread
                            // Check if this is a simple atomic match or an
                            // extended path
                            if path.len() == 1 && path[0] == th.env {
                                // Simple atomic match - keep existing path and
                                // environment
                                // (The pattern matched at the current position)
                            } else {
                                // Extended path from CBOR pattern - use the
                                // full extended path
                                th.path = path.clone();
                                if let Some(last_env) = path.last() {
                                    th.env = last_env.clone();
                                }
                            }

                            // Add distributed captures for this path to the
                            // current thread
                            if let Some(path_captures) =
                                distributed_captures.get(i)
                            {
                                for (name, capture_paths) in path_captures {
                                    if let Some(capture_idx) = prog
                                        .capture_names
                                        .iter()
                                        .position(|n| n == name)
                                    {
                                        if capture_idx < th.captures.len() {
                                            th.captures[capture_idx]
                                                .extend(capture_paths.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Spawn threads for remaining paths in reverse order to
                    // preserve original order when
                    // processed from stack (LIFO)
                    for (path_idx, path) in paths_vec
                        .iter()
                        .enumerate()
                        .skip(1)
                        .collect::<Vec<_>>()
                        .into_iter()
                        .rev()
                    {
                        let mut fork = th.clone();
                        // Reset captures for the fork to avoid duplication
                        for capture_vec in &mut fork.captures {
                            capture_vec.clear();
                        }
                        // For additional paths, always use the full path
                        // since these are separate matches
                        fork.path = path.clone();
                        if let Some(last_env) = path.last() {
                            fork.env = last_env.clone();
                        }

                        // Add distributed captures for this path to the fork
                        if let Some(path_captures) =
                            distributed_captures.get(path_idx)
                        {
                            for (name, capture_paths) in path_captures {
                                if let Some(capture_idx) = prog
                                    .capture_names
                                    .iter()
                                    .position(|n| n == name)
                                {
                                    if capture_idx < fork.captures.len() {
                                        fork.captures[capture_idx]
                                            .extend(capture_paths.clone());
                                    }
                                }
                            }
                        }

                        stack.push(fork);
                    }
                }
                MatchStructure(idx) => {
                    // Use the structure pattern's direct matcher, not the
                    // compiled pattern
                    let structure_paths =
                        if let crate::pattern::Pattern::Structure(sp) =
                            &prog.literals[idx]
                        {
                            // Call the structure pattern's direct paths method
                            sp.paths(&th.env)
                        } else {
                            panic!(
                                "MatchStructure used with non-structure pattern"
                            );
                        };

                    if structure_paths.is_empty() {
                        break;
                    }

                    th.pc += 1; // Advance to next instruction

                    // Spawn a new thread for each path found by the structure
                    // pattern
                    for (i, structure_path) in
                        structure_paths.into_iter().enumerate()
                    {
                        if i == 0 {
                            // Use the first path for the current thread
                            th.path = structure_path.clone();
                            if let Some(last_env) = structure_path.last() {
                                th.env = last_env.clone();
                            }
                        } else {
                            // Spawn new threads for the remaining paths
                            let mut fork = th.clone();
                            fork.path = structure_path.clone();
                            if let Some(last_env) = structure_path.last() {
                                fork.env = last_env.clone();
                            }
                            stack.push(fork);
                        }
                    }
                }
                Split { a, b } => {
                    let mut fork = th.clone();
                    fork.pc = a;
                    stack.push(fork);
                    th.pc = b;
                }
                Jump(t) => th.pc = t,
                PushAxis(axis) => {
                    th.pc += 1;
                    for (child, _edge) in axis.children(&th.env) {
                        let mut fork = th.clone();
                        fork.env = child.clone();
                        fork.path.push(child);
                        stack.push(fork);
                    }
                    break; // parent path stops here
                }
                Pop => {
                    th.path.pop();
                    th.pc += 1;
                }
                Save => {
                    out.push((th.path.clone(), th.captures.clone()));
                    produced = true;
                    th.pc += 1;
                }
                Accept => {
                    out.push((th.path.clone(), th.captures.clone()));
                    produced = true;
                    break;
                }
                Search { pat_idx, ref capture_map } => {
                    let inner = &prog.literals[pat_idx];
                    let (found_paths, caps) =
                        inner.paths_with_captures(&th.env);

                    if !found_paths.is_empty() {
                        produced = true;
                        for found_path in found_paths {
                            let mut result_path = th.path.clone();
                            if let Some(first) = found_path.first() {
                                if first == &th.env {
                                    result_path
                                        .extend(found_path.into_iter().skip(1));
                                } else {
                                    result_path.extend(found_path);
                                }
                            }

                            let mut result_caps = th.captures.clone();
                            for (name, idx) in capture_map {
                                if let Some(pths) = caps.get(name) {
                                    result_caps[*idx].extend(pths.clone());
                                }
                            }
                            let digests: Vec<_> = result_path
                                .iter()
                                .map(|e| e.digest().into_owned())
                                .collect();
                            if th.seen.insert(digests) {
                                out.push((result_path, result_caps));
                            }
                        }
                    }

                    // 2) always walk children (same traversal as
                    //    Envelope::walk)
                    // Collect all children first, then push in reverse order to
                    // maintain the same traversal order as
                    // the original recursive implementation
                    // Build child list following the same structure order as
                    // `Envelope::walk_structure`. This ensures every envelope
                    // is visited exactly once.
                    let mut all_children = Vec::new();
                    use bc_envelope::base::envelope::EnvelopeCase::*;
                    match th.env.case() {
                        Node { subject, assertions, .. } => {
                            all_children.push(subject.clone());
                            for assertion in assertions {
                                all_children.push(assertion.clone());
                            }
                        }
                        Wrapped { envelope, .. } => {
                            all_children.push(envelope.clone());
                        }
                        Assertion(assertion) => {
                            all_children.push(assertion.predicate().clone());
                            all_children.push(assertion.object().clone());
                        }
                        _ => {}
                    }

                    // Push child threads in reverse order so stack processes
                    // them in forward order
                    for child in all_children.into_iter().rev() {
                        let mut fork = th.clone();
                        fork.env = child.clone();
                        fork.path.push(child);
                        // fork continues with same PC to re-execute Search at
                        // child
                        stack.push(fork);
                    }

                    // This thread is done - either it emitted results or it
                    // didn't
                    break;
                }
                ExtendTraversal => {
                    // Save the current path and switch to the last envelope for
                    // the rest of the traversal
                    if let Some(last_env) = th.path.last().cloned() {
                        th.saved_paths.push(th.path.clone());
                        th.env = last_env.clone();
                        th.path = vec![last_env]; // Start fresh path from the last envelope
                    }
                    th.pc += 1;
                }
                CombineTraversal => {
                    // Combine saved path with current path, extending the saved
                    // path
                    if let Some(saved_path) = th.saved_paths.pop() {
                        let mut combined = saved_path.clone();

                        // If the current path starts with the same envelope as
                        // the saved path ends with,
                        // skip the first element to avoid duplication.
                        // Otherwise, append the whole current path.
                        if let (Some(saved_last), Some(current_first)) =
                            (saved_path.last(), th.path.first())
                        {
                            if saved_last == current_first {
                                // Skip first element to avoid duplication
                                combined
                                    .extend(th.path.iter().skip(1).cloned());
                            } else {
                                // Append whole current path
                                combined.extend(th.path.iter().cloned());
                            }
                        } else {
                            // Append whole current path if one of the paths is
                            // empty
                            combined.extend(th.path.iter().cloned());
                        }

                        th.path = combined;
                    }
                    th.pc += 1;
                }
                Repeat { pat_idx, quantifier } => {
                    let pat = &prog.literals[pat_idx];
                    let results =
                        repeat_paths(pat, &th.env, &th.path, quantifier);
                    if results.is_empty() {
                        break;
                    }
                    // Try each repetition count in order. `run_thread` fully
                    // explores all branches for that count and returns `true`
                    // if it yields any paths. Once one count succeeds we stop
                    // trying further counts, emulating regex greedy/lazy
                    // semantics while still returning all matching paths for
                    // the chosen count.
                    let next_pc = th.pc + 1;
                    let mut success = false;
                    for (env_after, path_after) in results {
                        let mut fork = th.clone();
                        fork.pc = next_pc;
                        fork.env = env_after;
                        fork.path = path_after;
                        if run_thread(prog, fork, out) {
                            produced = true;
                            success = true;
                            break;
                        }
                    }
                    if !success {
                        // None of the repetition counts allowed the rest to
                        // match
                    }
                    break;
                }
                NavigateSubject => {
                    // If the current envelope is a node, navigate to its
                    // subject and update the path.
                    if th.env.is_node() {
                        let subject = th.env.subject();
                        th.env = subject.clone();
                        th.path.push(subject);
                    }
                    th.pc += 1;
                }
                NotMatch { pat_idx } => {
                    // Check if the pattern matches. If it doesn't match, the
                    // NOT pattern succeeds. If it does
                    // match, the NOT pattern fails and we kill this thread.

                    // For atomic patterns, use atomic_paths for efficiency
                    let pattern = &prog.literals[pat_idx];
                    let pattern_matches = match pattern {
                        crate::pattern::Pattern::Leaf(_) => {
                            pattern.matches(&th.env)
                        }
                        crate::pattern::Pattern::Structure(_) => {
                            pattern.matches(&th.env)
                        }
                        crate::pattern::Pattern::Meta(_) => {
                            pattern.matches(&th.env)
                        }
                    };

                    if pattern_matches {
                        // Inner pattern matches, so NOT pattern fails - kill
                        // this thread
                        break;
                    } else {
                        // Inner pattern doesn't match, so NOT pattern succeeds
                        // - continue
                        th.pc += 1;
                    }
                }
                CaptureStart(id) => {
                    if th.capture_stack.len() > id {
                        th.capture_stack[id].push(th.path.len() - 1);
                    }
                    th.pc += 1;
                }
                CaptureEnd(id) => {
                    if th.capture_stack.len() > id {
                        if let Some(start_idx) = th.capture_stack[id].pop() {
                            if th.captures.len() > id {
                                let mut end = th.path.len();
                                if let Some(Instr::ExtendTraversal) =
                                    prog.code.get(th.pc + 1)
                                {
                                    end = end.saturating_sub(1);
                                }
                                let cap = th.path[start_idx..end].to_vec();
                                th.captures[id].push(cap);
                            }
                        }
                    }
                    th.pc += 1;
                }
            }
        }
    }
    produced
}

/// Execute `prog` starting at `root`.  Every time `SAVE` or `ACCEPT` executes,
/// the current `path` is pushed into the result.
pub fn run(
    prog: &Program,
    root: &Envelope,
) -> Vec<(Path, std::collections::HashMap<String, Vec<Path>>)> {
    let mut out = Vec::new();
    let start = Thread {
        pc: 0,
        env: root.clone(),
        path: vec![root.clone()],
        saved_paths: Vec::new(),
        captures: vec![Vec::new(); prog.capture_names.len()],
        capture_stack: vec![Vec::new(); prog.capture_names.len()],
        seen: std::collections::HashSet::new(),
    };
    run_thread(prog, start, &mut out);
    out.into_iter()
        .map(|(path, caps)| {
            let mut map = std::collections::HashMap::new();
            for (i, paths) in caps.into_iter().enumerate() {
                if !paths.is_empty() {
                    map.insert(prog.capture_names[i].clone(), paths);
                }
            }
            (path, map)
        })
        .collect()
}
