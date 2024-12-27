#[test]
fn test_target_tuple() {
    assert_ne!(
        testing_integration_rustc_private::host(),
        "unknown-unknown-unknown"
    );
}
