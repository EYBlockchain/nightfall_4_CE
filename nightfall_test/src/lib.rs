pub mod run_tests;
pub mod test;
pub mod test_settings;
pub mod webhook;

#[derive(Debug)]
pub struct TestError {
    message: String,
}
impl TestError {
    fn new(message: String) -> Self {
        TestError {
            message: message.to_string(),
        }
    }
}
impl std::fmt::Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
