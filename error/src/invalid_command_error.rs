#[derive(Debug)]
pub struct InvalidCommandError;

impl std::fmt::Display for InvalidCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "InvalidCommandError")
    }
}

impl std::error::Error for InvalidCommandError {
    fn description(&self) -> &str {
        "Command is invalid"
    }
}
