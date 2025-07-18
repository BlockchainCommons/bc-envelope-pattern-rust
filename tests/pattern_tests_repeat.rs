mod common;

use bc_envelope::prelude::*;
use bc_envelope_pattern::{Matcher, Path, Pattern, Reluctance, format_paths};
use indoc::indoc;

fn fold(string: &str) -> Envelope {
    let chars: Vec<String> = string.chars().map(|c| c.to_string()).collect();
    let mut it = chars.into_iter().enumerate().rev();
    let (index, c) = it.next().unwrap();
    let mut env = Envelope::new_assertion(index, c);
    for (index, c) in it {
        let obj = Envelope::new(c.clone())
            .add_assertion_envelope(env)
            .unwrap();
        env = Envelope::new_assertion(index, obj);
    }
    Envelope::unit().add_assertion_envelope(env).unwrap()
}

fn unfold(env: impl AsRef<Envelope>) -> String {
    let mut result = String::new();
    let mut env = Some(env.as_ref().clone());
    while let Some(e) = env {
        if e.is_assertion() {
            let object = e.as_object().unwrap();
            let c: String = object.extract_subject().unwrap();
            result.push_str(&c);
            env = object.assertions().first().cloned();
        } else {
            env = e.assertions().first().cloned();
        }
    }
    result
}

#[test]
fn test_fold() {
    bc_envelope::register_tags();

    let s = "hello";
    let folded = fold(s);

    #[rustfmt::skip]
    let expected = indoc! {r#"
        '' [
            0: "h" [
                1: "e" [
                    2: "l" [
                        3: "l" [
                            4: "o"
                        ]
                    ]
                ]
            ]
        ]
    "#}.trim();
    assert_actual_expected!(folded.format(), expected);

    #[rustfmt::skip]
    let expected =  indoc! {r#"
        b229d3cb NODE
            934312d6 subj ''
            1b47f7a1 ASSERTION
                6e340b9c pred 0
                dc1d9ddc obj NODE
                    70a0d519 subj "h"
                    354b5ed3 ASSERTION
                        4bf5122f pred 1
                        a899ff63 obj NODE
                            f9a00f43 subj "e"
                            7e272ce6 ASSERTION
                                dbc1b4c9 pred 2
                                bff05dca obj NODE
                                    63518250 subj "l"
                                    d71e5aaf ASSERTION
                                        084fed08 pred 3
                                        73381991 obj NODE
                                            63518250 subj "l"
                                            7c92231b ASSERTION
                                                e52d9c50 pred 4
                                                2dd41130 obj "o"
    "#}.trim();
    assert_actual_expected!(folded.tree_format(), expected);

    let unfolded = unfold(folded);
    assert_eq!(unfolded, s);
}

#[test]
fn repeat_test() {
    bc_envelope::register_tags();

    let s = "hello";
    let env = fold(s);

    let pattern = Pattern::traverse(vec![Pattern::any_assertion()]);
    assert_eq!(format!("{}", pattern), "assert");
    let paths = pattern.paths(&env);
    assert_eq!(unfold(paths[0].last().unwrap()), s);

    let assertion_object_pattern = Pattern::traverse(vec![
        Pattern::any_assertion(),
        Pattern::any_object(),
    ]);
    assert_eq!(format!("{}", assertion_object_pattern), "assert -> obj");

    let pattern =
        Pattern::repeat(assertion_object_pattern, 3..=3, Reluctance::Greedy);
    assert_eq!(format!("{}", pattern), "(assert -> obj){3}");
    let paths = pattern.paths(&env);
    assert_eq!(paths.len(), 1);

    let path = &paths[0];
    assert_eq!(transpose(path), "hel");
    assert_eq!(unfold(path.last().unwrap()), "lo");
}

#[test]
fn test_repeat_2() {
    let str = "AabBbabB";
    let env = fold(str);

    let seq_a = Pattern::traverse(vec![
        Pattern::assertion_with_object(Pattern::text("A")),
        Pattern::any_object(),
    ]);
    assert_eq!(format!("{}", seq_a), r#"assertobj("A") -> obj"#);

    let seq_any = Pattern::traverse(vec![
        Pattern::any_assertion(),
        Pattern::any_object(),
    ]);
    assert_eq!(format!("{}", seq_any), r#"assert -> obj"#);

    let seq_b = Pattern::traverse(vec![
        Pattern::assertion_with_object(Pattern::text("B")),
        Pattern::any_object(),
    ]);
    assert_eq!(format!("{}", seq_b), r#"assertobj("B") -> obj"#);

    let pat = |mode| {
        Pattern::traverse(vec![
            seq_a.clone(),
            Pattern::repeat(seq_any.clone(), .., mode),
            seq_b.clone(),
        ])
    };

    let pattern = pat(Reluctance::Greedy);
    assert_eq!(
        format!("{}", pattern),
        r#"assertobj("A") -> obj -> (assert -> obj)* -> assertobj("B") -> obj"#
    );
    let paths = pattern.paths(&env);
    assert_eq!(paths.len(), 1);
    assert_eq!(transpose(&paths[0]), "AabBbabB");

    let pattern = pat(Reluctance::Lazy);
    assert_eq!(
        format!("{}", pattern),
        r#"assertobj("A") -> obj -> (assert -> obj)*? -> assertobj("B") -> obj"#
    );
    let paths = pattern.paths(&env);
    assert_eq!(paths.len(), 1);
    assert_eq!(transpose(&paths[0]), "AabB");

    let pattern = pat(Reluctance::Possessive);
    assert_eq!(
        format!("{}", pattern),
        r#"assertobj("A") -> obj -> (assert -> obj)*+ -> assertobj("B") -> obj"#
    );
    let paths = pattern.paths(&env);
    assert_eq!(paths.len(), 0);
}

fn transpose(path: impl AsRef<Path>) -> String {
    path.as_ref()
        .iter()
        .filter_map(|e| e.subject().as_text())
        .collect::<Vec<_>>()
        .join("")
}

fn wrap_n(mut env: Envelope, n: usize) -> Envelope {
    for _ in 0..n {
        env = env.wrap();
    }
    env
}

#[test]
fn repeat_any_greedy() {
    let pat = Pattern::traverse(vec![
        Pattern::repeat(Pattern::unwrap(), .., Reluctance::Greedy),
        Pattern::any_cbor(),
    ]);

    let env = wrap_n(Envelope::new(42), 4);
    let paths = pat.paths(&env);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        3a0b1e87 WRAPPED { { { { 42 } } } }
            75659622 WRAPPED { { { 42 } } }
                81bb1f5e WRAPPED { { 42 } }
                    58b1ac6a WRAPPED { 42 }
                        7f83f7bd LEAF 42
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);
}

#[test]
fn repeat_any_lazy() {
    let env = wrap_n(Envelope::new(42), 4);
    let pat = Pattern::traverse(vec![
        Pattern::repeat(Pattern::unwrap(), .., Reluctance::Lazy),
        Pattern::any_cbor(),
    ]);
    let paths = pat.paths(&env);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        3a0b1e87 WRAPPED { { { { 42 } } } }
            75659622 WRAPPED { { { 42 } } }
                81bb1f5e WRAPPED { { 42 } }
                    58b1ac6a WRAPPED { 42 }
                        7f83f7bd LEAF 42
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);
}

#[test]
fn repeat_any_possessive() {
    let env = wrap_n(Envelope::new(42), 4);
    let pat = Pattern::traverse(vec![
        Pattern::repeat(Pattern::unwrap(), .., Reluctance::Possessive),
        Pattern::any_cbor(),
    ]);
    let paths = pat.paths(&env);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        3a0b1e87 WRAPPED { { { { 42 } } } }
            75659622 WRAPPED { { { 42 } } }
                81bb1f5e WRAPPED { { 42 } }
                    58b1ac6a WRAPPED { 42 }
                        7f83f7bd LEAF 42
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);
}

#[test]
fn repeat_some_greedy() {
    let env = wrap_n(Envelope::new(42), 3);
    let pat = Pattern::traverse(vec![
        Pattern::repeat(Pattern::unwrap(), 1.., Reluctance::Greedy),
        Pattern::any_cbor(),
    ]);
    let paths = pat.paths(&env);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        75659622 WRAPPED { { { 42 } } }
            81bb1f5e WRAPPED { { 42 } }
                58b1ac6a WRAPPED { 42 }
                    7f83f7bd LEAF 42
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);
}

#[test]
fn repeat_some_lazy() {
    let env = wrap_n(Envelope::new(42), 3);
    let pat = Pattern::traverse(vec![
        Pattern::repeat(Pattern::unwrap(), 1.., Reluctance::Lazy),
        Pattern::any_cbor(),
    ]);
    let paths = pat.paths(&env);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        75659622 WRAPPED { { { 42 } } }
            81bb1f5e WRAPPED { { 42 } }
                58b1ac6a WRAPPED { 42 }
                    7f83f7bd LEAF 42
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);
}

#[test]
fn repeat_some_possessive() {
    let env = wrap_n(Envelope::new(42), 3);
    let pat = Pattern::traverse(vec![
        Pattern::repeat(Pattern::unwrap(), 1.., Reluctance::Possessive),
        Pattern::any_cbor(),
    ]);
    let paths = pat.paths(&env);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        75659622 WRAPPED { { { 42 } } }
            81bb1f5e WRAPPED { { 42 } }
                58b1ac6a WRAPPED { 42 }
                    7f83f7bd LEAF 42
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);
}

#[test]
fn repeat_optional_greedy() {
    let pat = Pattern::traverse(vec![
        Pattern::repeat(Pattern::unwrap(), 0..=1, Reluctance::Greedy),
        Pattern::any_cbor(),
    ]);
    let paths = pat.paths(&wrap_n(Envelope::new(42), 0));
    #[rustfmt::skip]
    let expected = indoc! {r#"
        7f83f7bd LEAF 42
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    let paths = pat.paths(&wrap_n(Envelope::new(42), 1));
    #[rustfmt::skip]
    let expected = indoc! {r#"
        58b1ac6a WRAPPED { 42 }
            7f83f7bd LEAF 42
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);
}

#[test]
fn repeat_optional_lazy() {
    let pat = Pattern::traverse(vec![
        Pattern::repeat(Pattern::unwrap(), 0..=1, Reluctance::Lazy),
        Pattern::any_cbor(),
    ]);
    let paths = pat.paths(&wrap_n(Envelope::new(42), 0));
    #[rustfmt::skip]
    let expected = indoc! {r#"
        7f83f7bd LEAF 42
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);
    let paths = pat.paths(&wrap_n(Envelope::new(42), 1));
    #[rustfmt::skip]
    let expected = indoc! {r#"
        58b1ac6a WRAPPED { 42 }
            7f83f7bd LEAF 42
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);
}

#[test]
fn repeat_optional_possessive() {
    let pat = Pattern::traverse(vec![
        Pattern::repeat(Pattern::unwrap(), 0..=1, Reluctance::Possessive),
        Pattern::any_cbor(),
    ]);
    let paths = pat.paths(&wrap_n(Envelope::new(42), 0));
    #[rustfmt::skip]
    let expected = indoc! {r#"
        7f83f7bd LEAF 42
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);
    let paths = pat.paths(&wrap_n(Envelope::new(42), 1));
    #[rustfmt::skip]
    let expected = indoc! {r#"
        58b1ac6a WRAPPED { 42 }
            7f83f7bd LEAF 42
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);
}

#[test]
fn repeat_range_greedy() {
    let pat = Pattern::traverse(vec![
        Pattern::repeat(Pattern::unwrap(), 2..=3, Reluctance::Greedy),
        Pattern::any_cbor(),
    ]);
    let env = wrap_n(Envelope::new(42), 3);
    assert!(pat.matches(&env));
    let paths = pat.paths(&env);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        75659622 WRAPPED { { { 42 } } }
            81bb1f5e WRAPPED { { 42 } }
                58b1ac6a WRAPPED { 42 }
                    7f83f7bd LEAF 42
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);
}

#[test]
fn repeat_range_lazy() {
    let pat = Pattern::traverse(vec![
        Pattern::repeat(Pattern::unwrap(), 2..=3, Reluctance::Lazy),
        Pattern::any_cbor(),
    ]);
    let env = wrap_n(Envelope::new(42), 3);
    let paths = pat.paths(&env);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        75659622 WRAPPED { { { 42 } } }
            81bb1f5e WRAPPED { { 42 } }
                58b1ac6a WRAPPED { 42 }
                    7f83f7bd LEAF 42
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);
}

#[test]
fn repeat_range_possessive() {
    let pat = Pattern::traverse(vec![
        Pattern::repeat(Pattern::unwrap(), 2..=3, Reluctance::Possessive),
        Pattern::any_cbor(),
    ]);
    let env = wrap_n(Envelope::new(42), 3);
    let paths = pat.paths(&env);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        75659622 WRAPPED { { { 42 } } }
            81bb1f5e WRAPPED { { 42 } }
                58b1ac6a WRAPPED { 42 }
                    7f83f7bd LEAF 42
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);
}

#[test]
fn repeat_any_modes() {
    let env = wrap_n(Envelope::new("data"), 2);

    let pat = |mode| {
        Pattern::traverse(vec![
            Pattern::repeat(Pattern::unwrap(), 0.., mode),
            Pattern::wrapped(),
            Pattern::unwrap(),
            Pattern::text("data"),
        ])
    };

    let greedy_paths = pat(Reluctance::Greedy).paths(&env);
    let lazy_paths = pat(Reluctance::Lazy).paths(&env);
    let possessive_paths = pat(Reluctance::Possessive).paths(&env);

    assert_eq!(greedy_paths, lazy_paths);
    assert!(possessive_paths.is_empty());

    #[rustfmt::skip]
    let expected = indoc! {r#"
        ee8cade0 WRAPPED { { "data" } }
            febc1555 WRAPPED { "data" }
                e909da9a LEAF "data"
    "#}.trim();
    assert_actual_expected!(format_paths(&greedy_paths), expected);
}

#[test]
fn repeat_optional_modes() {
    let env = wrap_n(Envelope::new(42), 1);

    let pat = |mode| {
        Pattern::traverse(vec![
            Pattern::repeat(Pattern::unwrap(), 0..=1, mode),
            Pattern::number(42),
        ])
    };

    let greedy_paths = pat(Reluctance::Greedy).paths(&env);
    let expected = indoc! {r#"
        58b1ac6a WRAPPED { 42 }
            7f83f7bd LEAF 42
    "#}
    .trim();
    assert_actual_expected!(format_paths(&greedy_paths), expected);

    let lazy_paths = pat(Reluctance::Lazy).paths(&env);
    let expected = indoc! {r#"
        58b1ac6a WRAPPED { 42 }
            7f83f7bd LEAF 42
    "#}
    .trim();
    assert_actual_expected!(format_paths(&lazy_paths), expected);

    let possessive_paths = pat(Reluctance::Possessive).paths(&env);
    let expected = indoc! {r#"
        58b1ac6a WRAPPED { 42 }
            7f83f7bd LEAF 42
    "#}
    .trim();
    assert_actual_expected!(format_paths(&possessive_paths), expected);
}

#[test]
fn repeat_some_order() {
    let env = wrap_n(Envelope::new("x"), 2);

    #[rustfmt::skip]
    let expected = indoc! {r#"
        06bb2465 WRAPPED
            70b5f17d cont WRAPPED
                5e85370e cont "x"
    "#}.trim();
    assert_actual_expected!(env.tree_format(), expected);

    let pat = |mode| {
        Pattern::traverse(vec![
            Pattern::repeat(Pattern::unwrap(), 1.., mode),
            Pattern::any_subject(),
        ])
    };

    let greedy_paths = pat(Reluctance::Greedy).paths(&env);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        06bb2465 WRAPPED { { "x" } }
            70b5f17d WRAPPED { "x" }
                5e85370e LEAF "x"
    "#}.trim();
    assert_actual_expected!(format_paths(&greedy_paths), expected);

    let lazy_paths = pat(Reluctance::Lazy).paths(&env);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        06bb2465 WRAPPED { { "x" } }
            70b5f17d WRAPPED { "x" }
    "#}.trim();
    assert_actual_expected!(format_paths(&lazy_paths), expected);

    let possessive_paths = pat(Reluctance::Possessive).paths(&env);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        06bb2465 WRAPPED { { "x" } }
            70b5f17d WRAPPED { "x" }
                5e85370e LEAF "x"
    "#}.trim();
    assert_actual_expected!(format_paths(&possessive_paths), expected);
}

#[test]
fn repeat_range_order() {
    let env = wrap_n(Envelope::new("x"), 4);

    let pat = |mode| {
        Pattern::traverse(vec![
            Pattern::repeat(Pattern::unwrap(), 2..=3, mode),
            Pattern::any_subject(),
        ])
    };

    let greedy_paths = pat(Reluctance::Greedy).paths(&env);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        88e28c8b WRAPPED { { { { "x" } } } }
            79962374 WRAPPED { { { "x" } } }
                06bb2465 WRAPPED { { "x" } }
                    70b5f17d WRAPPED { "x" }
    "#}.trim();
    assert_actual_expected!(format_paths(&greedy_paths), expected);

    let lazy_paths = pat(Reluctance::Lazy).paths(&env);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        88e28c8b WRAPPED { { { { "x" } } } }
            79962374 WRAPPED { { { "x" } } }
                06bb2465 WRAPPED { { "x" } }
    "#}.trim();
    assert_actual_expected!(format_paths(&lazy_paths), expected);

    let possessive_paths = pat(Reluctance::Possessive).paths(&env);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        88e28c8b WRAPPED { { { { "x" } } } }
            79962374 WRAPPED { { { "x" } } }
                06bb2465 WRAPPED { { "x" } }
                    70b5f17d WRAPPED { "x" }
    "#}.trim();
    assert_actual_expected!(format_paths(&possessive_paths), expected);
}

#[test]
#[ignore]
fn test_repeat() {
    // A pattern that matches zero or more `unwrap` elements leading to a
    // `node`.
    let pat = Pattern::parse("(unwrap)*>node").unwrap();

    let env = Envelope::new("Alice");
    // There is no `node` in the envelope, so the pattern should not match.
    assert!(!pat.matches(&env));

    let env = env.add_assertion("knows", "Bob");
    let paths = pat.paths(&env);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        8955db5e NODE "Alice" [ "knows": "Bob" ]
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    let wrapped_env = env.wrap();
    let paths = pat.paths(&wrapped_env);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        fd881a24 WRAPPED { "Alice" [ "knows": "Bob" ] }
            8955db5e NODE "Alice" [ "knows": "Bob" ]
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // A pattern that matches zero or more `wrapped` elements leading to a
    // `node`.
    let pat = Pattern::parse("(wrapped)*>node").unwrap();
    // Does not match, because even though the `wrapped` part matches, it
    // does not make progress into the wrapped node to get to the `node`.
    assert!(!pat.matches(&wrapped_env));

    let pat = Pattern::parse("@cap((wrapped)*)>unwrap>node").unwrap();
    let (paths, captures) = pat.paths_with_captures(&wrapped_env);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        fd881a24 WRAPPED { "Alice" [ "knows": "Bob" ] }
            8955db5e NODE "Alice" [ "knows": "Bob" ]
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);
    let caps = captures.get("cap").unwrap();
    assert_eq!(caps.len(), 1);
    #[rustfmt::skip]
    let expected_cap = indoc! {r#"
        fd881a24 WRAPPED { "Alice" [ "knows": "Bob" ] }
    "#}.trim();
    assert_actual_expected!(format_paths(caps), expected_cap);

    let wrapped_env = wrapped_env.wrap();
    let pat = Pattern::parse("@cap((wrapped>unwrap)*)>node").unwrap();
    let (paths, captures) = pat.paths_with_captures(&wrapped_env);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        3defda74 WRAPPED { { "Alice" [ "knows": "Bob" ] } }
            fd881a24 WRAPPED { "Alice" [ "knows": "Bob" ] }
                8955db5e NODE "Alice" [ "knows": "Bob" ]
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);
    let caps = captures.get("cap").unwrap();
    assert_eq!(caps.len(), 1);
    #[rustfmt::skip]
    let expected_cap = indoc! {r#"
        3defda74 WRAPPED { { "Alice" [ "knows": "Bob" ] } }
            fd881a24 WRAPPED { "Alice" [ "knows": "Bob" ] }
    "#}.trim();
    assert_actual_expected!(format_paths(caps), expected_cap);
}

#[test]
fn test_capture() {
    let env = Envelope::new("Alice")
        .add_assertion("knows", "Bob")
        .wrap()
        .wrap();

    #[rustfmt::skip]
    let expected = indoc! {r#"
        3defda74 WRAPPED
            fd881a24 cont WRAPPED
                8955db5e cont NODE
                    13941b48 subj "Alice"
                    78d666eb ASSERTION
                        db7dd21c pred "knows"
                        13b74194 obj "Bob"
    "#}.trim();
    assert_actual_expected!(env.tree_format(), expected);

    // Pattern only captures `wrapped` elements leading to a `node`,
    // but not the `node` itself.
    let pat = Pattern::parse("@cap( (wrapped -> unwrap)* ) -> node").unwrap();
    let (paths, captures) = pat.paths_with_captures(&env);
    // Pattern matches the `wrapped` elements leading to the `node`,
    // and the `node` itself.
    #[rustfmt::skip]
    let expected = indoc! {r#"
        3defda74 WRAPPED { { "Alice" [ "knows": "Bob" ] } }
            fd881a24 WRAPPED { "Alice" [ "knows": "Bob" ] }
                8955db5e NODE "Alice" [ "knows": "Bob" ]
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);
    let caps = captures.get("cap").unwrap();
    assert_eq!(caps.len(), 1);
    // The capture contains the `wrapped` elements leading to the `node`, but
    // not the `NODE` itself.
    #[rustfmt::skip]
    let expected_cap = indoc! {r#"
        3defda74 WRAPPED { { "Alice" [ "knows": "Bob" ] } }
            fd881a24 WRAPPED { "Alice" [ "knows": "Bob" ] }
    "#}.trim();
    assert_actual_expected!(format_paths(caps), expected_cap);
}
