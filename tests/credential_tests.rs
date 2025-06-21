mod common;

use bc_envelope::prelude::*;
use bc_envelope_pattern::{Matcher, Pattern};
use indoc::indoc;

use crate::common::{
    pattern_utils::{format_paths_opt, FormatPathOpts},
    test_data::{credential, redacted_credential},
};

#[test]
fn test_credential() {
    let env = credential();
    #[rustfmt::skip]
    assert_actual_expected!(env.format(), indoc! {r#"
        {
            ARID(4676635a) [
                'isA': "Certificate of Completion"
                "certificateNumber": "123-456-789"
                "continuingEducationUnits": 1
                "expirationDate": 2028-01-01
                "firstName": "James"
                "issueDate": 2020-01-01
                "lastName": "Maxwell"
                "photo": "This is James Maxwell's photo."
                "professionalDevelopmentHours": 15
                "subject": "RF and Microwave Engineering"
                "topics": ["Subject 1", "Subject 2"]
                'controller': "Example Electrical Engineering Board"
                'issuer': "Example Electrical Engineering Board"
            ]
        } [
            'note': "Signed by Example Electrical Engineering Board"
            'signed': Signature
        ]
    "#}.trim());
    #[rustfmt::skip]
    assert_actual_expected!(env.format_flat(), indoc! {r#"
        { ARID(4676635a) [ 'isA': "Certificate of Completion", "certificateNumber": "123-456-789", "continuingEducationUnits": 1, "expirationDate": 2028-01-01, "firstName": "James", "issueDate": 2020-01-01, "lastName": "Maxwell", "photo": "This is James Maxwell's photo.", "professionalDevelopmentHours": 15, "subject": "RF and Microwave Engineering", "topics": ["Subject 1", "Subject 2"], 'controller': "Example Electrical Engineering Board", 'issuer': "Example Electrical Engineering Board" ] } [ 'note': "Signed by Example Electrical Engineering Board", 'signed': Signature ]
    "#}.trim());
    #[rustfmt::skip]
    assert_actual_expected!(env.tree_format(), indoc! {r#"
        0b721f78 NODE
            397a2d4c subj WRAPPED
                8122ffa9 cont NODE
                    10d3de01 subj ARID(4676635a)
                    1f9ff098 ASSERTION
                        9e3bff3a pred "certificateNumber"
                        21c21808 obj "123-456-789"
                    36c254d0 ASSERTION
                        6e5d379f pred "expirationDate"
                        639ae9bf obj 2028-01-01
                    3c114201 ASSERTION
                        5f82a16a pred "lastName"
                        fe4d5230 obj "Maxwell"
                    4a9b2e4d ASSERTION
                        222afe69 pred "issueDate"
                        cb67f31d obj 2020-01-01
                    4d67bba0 ASSERTION
                        2be2d79b pred 'isA'
                        051beee6 obj "Certificate of Completion"
                    5171cbaf ASSERTION
                        3976ef74 pred "photo"
                        231b8527 obj "This is James Maxwell's photo."
                    54b3e1e7 ASSERTION
                        f13aa855 pred "professionalDevelopmentHours"
                        dc0e9c36 obj 15
                    5dc6d4e3 ASSERTION
                        4395643b pred "firstName"
                        d6d0b768 obj "James"
                    68895d8e ASSERTION
                        e6bf4dd3 pred "topics"
                        543fcc09 obj ["Subject 1", "Subject 2"]
                    8ec5e912 ASSERTION
                        2b191589 pred "continuingEducationUnits"
                        4bf5122f obj 1
                    9b3d4785 ASSERTION
                        af10ee92 pred 'controller'
                        f8489ac1 obj "Example Electrical Engineering Board"
                    caf5ced3 ASSERTION
                        8e4e62eb pred "subject"
                        202c10ef obj "RF and Microwave Engineering"
                    d3e0cc15 ASSERTION
                        6dd16ba3 pred 'issuer'
                        f8489ac1 obj "Example Electrical Engineering Board"
            46a02aaf ASSERTION
                d0e39e78 pred 'signed'
                34c14941 obj Signature
            e6d7fca0 ASSERTION
                0fcd6a39 pred 'note'
                f106bad1 obj "Signed by Example Electrical Engineering…"
    "#}.trim());
}

#[test]
fn test_parsed_search_text_or_number() {
    let env = credential();
    let pattern = Pattern::parse("SEARCH(ASSERTOBJ(TEXT|NUMBER))").unwrap();
    let paths = pattern.paths(&env);
    assert_eq!(paths.len(), 11);
}

#[test]
fn test_parsed_firstname_capture() {
    let env = credential();
    let pattern_str =
        r#"SEARCH(ASSERTPRED(TEXT("firstName"))>OBJ(TEXT("James")))"#;
    let pattern = Pattern::parse(pattern_str).unwrap();
    let paths = pattern.paths(&env);
    assert_eq!(paths.len(), 1);
}

#[test]
fn test_search_capture_propagation() {
    let env = credential();
    let pattern_str =
        r#"SEARCH(@cap(ASSERTPRED(TEXT("firstName"))>OBJ(TEXT("James"))))"#;
    let pattern = Pattern::parse(pattern_str).unwrap();
    let (paths, caps) = pattern.paths_with_captures(&env);
    assert_eq!(paths.len(), 1);
    assert_eq!(caps.get("cap").unwrap().len(), 1);
}

#[test]
fn test_parsed_node_structure() {
    let env = credential();
    let pat = Pattern::parse("SEARCH(NODE({13}))").unwrap();
    let paths = pat.paths(&env);
    assert_eq!(paths.len(), 1);
}

#[test]
fn test_digest_and_not() {
    let env = credential();
    let digest = env.short_id(bc_envelope::DigestDisplayFormat::Short);
    let pattern_str = format!("DIGEST({})&(!OBSCURED)", digest);
    let pat = Pattern::parse(&pattern_str).unwrap();
    assert!(pat.matches(&env));
}

#[test]
#[ignore]
fn test_wrapped_repeat() {
    // See above for the full tree structure of the credential.
    let env = credential();

    // A pattern that matches zero or more `WRAPPED` elements leading to a
    // `NODE`.
    let pat = Pattern::parse("(WRAPPED)*>NODE").unwrap();
    let paths = pat.paths(&env);

    // The pattern should match both the outer node and its unwrapped subject
    // node since the `WRAPPED` repetition can consume the wrapper around the
    // subject.
    #[rustfmt::skip]
    let expected = indoc! {r#"
        0b721f78 NODE { ARID(4676635a) [ 'isA': "Certificate of Completion", "certifica…
            8122ffa9 NODE ARID(4676635a) [ 'isA': "Certificate of Completion", "certificate…
    "#}
    .trim();

    assert_actual_expected!(
        format_paths_opt(&paths, FormatPathOpts::default().max_length(80)),
        expected
    );
}

#[test]
#[ignore]
fn test_search_wrapped_repeat() {
    // See above for the full tree structure of the credential.
    let env = credential();
    // A pattern that searches every element in the tree for those
    // that start a path of zero or more `WRAPPED` elements leading to a `NODE`.
    let pat = Pattern::parse("SEARCH((WRAPPED)*>NODE)").unwrap();
    let paths = pat.paths(&env);
    // Every `NODE` in the tree should match the pattern, including the inner
    // `8122ffa9 NODE`. Consequently there are two matching paths: one that
    // descends through the wrapper and another that matches the inner node
    // directly.
    #[rustfmt::skip]
    let expected = indoc! {r#"
        0b721f78 NODE { ARID(4676635a) [ 'isA': "Certificate of Completion", "certifica…
            8122ffa9 NODE ARID(4676635a) [ 'isA': "Certificate of Completion", "certificate…
        0b721f78 NODE { ARID(4676635a) [ 'isA': "Certificate of Completion", "certifica…
            397a2d4c WRAPPED { ARID(4676635a) [ 'isA': "Certificate of Completion", "certif…
                8122ffa9 NODE ARID(4676635a) [ 'isA': "Certificate of Completion", "certificate…
    "#}
    .trim();

    assert_actual_expected!(
        format_paths_opt(&paths, FormatPathOpts::default().max_length(80)),
        expected
    );
}

#[test]
fn test_redacted_credential() {
    let env = redacted_credential();
    #[rustfmt::skip]
    assert_actual_expected!(env.format(), indoc! {r#"
        {
            ARID(4676635a) [
                'isA': "Certificate of Completion"
                "expirationDate": 2028-01-01
                "firstName": "James"
                "lastName": "Maxwell"
                "subject": "RF and Microwave Engineering"
                'issuer': "Example Electrical Engineering Board"
                ELIDED (7)
            ]
        } [
            'note': "Signed by Example Electrical Engineering Board"
            'signed': Signature
        ]
    "#}.trim());
    #[rustfmt::skip]
    assert_actual_expected!(env.tree_format(), indoc! {r#"
        0b721f78 NODE
            397a2d4c subj WRAPPED
                8122ffa9 cont NODE
                    10d3de01 subj ARID(4676635a)
                    1f9ff098 ELIDED
                    36c254d0 ASSERTION
                        6e5d379f pred "expirationDate"
                        639ae9bf obj 2028-01-01
                    3c114201 ASSERTION
                        5f82a16a pred "lastName"
                        fe4d5230 obj "Maxwell"
                    4a9b2e4d ELIDED
                    4d67bba0 ASSERTION
                        2be2d79b pred 'isA'
                        051beee6 obj "Certificate of Completion"
                    5171cbaf ELIDED
                    54b3e1e7 ELIDED
                    5dc6d4e3 ASSERTION
                        4395643b pred "firstName"
                        d6d0b768 obj "James"
                    68895d8e ELIDED
                    8ec5e912 ELIDED
                    9b3d4785 ELIDED
                    caf5ced3 ASSERTION
                        8e4e62eb pred "subject"
                        202c10ef obj "RF and Microwave Engineering"
                    d3e0cc15 ASSERTION
                        6dd16ba3 pred 'issuer'
                        f8489ac1 obj "Example Electrical Engineering Board"
            46a02aaf ASSERTION
                d0e39e78 pred 'signed'
                34c14941 obj Signature
            e6d7fca0 ASSERTION
                0fcd6a39 pred 'note'
                f106bad1 obj "Signed by Example Electrical Engineering…"
    "#}.trim());
}

#[test]
fn test_search_elided() {
    let env = redacted_credential();
    let pat = Pattern::parse("SEARCH(ELIDED)").unwrap();
    let paths = pat.paths(&env);
    assert_eq!(paths.len(), 7);
}
