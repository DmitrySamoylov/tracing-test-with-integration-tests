#[test]
#[tracing_test::traced_test]
fn test() {
    tracing::debug!("this is printed");
    tracing_test_with_integration_tests::print_something();
}
