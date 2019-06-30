#[derive(Debug)]
pub struct ParseCommandError;

impl std::fmt::Display for ParseCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ParseCommandError")
    }
}

impl std::error::Error for ParseCommandError {
    fn description(&self) -> &str {
        "Input can not be parsed to command"
    }
}
