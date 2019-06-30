use rust_chain_error::invalid_command_error::InvalidCommandError;
use rust_chain_error::parse_command_error::ParseCommandError;
use crate::simulator;

pub enum Commands {
    CreateAccount,
    Simulate,
}

impl Commands {
    pub fn parse(arg: Option<&String>) -> Result<Commands, ParseCommandError> {
        match arg {
            Some(v) => {
                let s = v.as_str();
                match s {
                    "create_account" => Ok(Commands::CreateAccount),
                    "simulate" => Ok(Commands::Simulate),
                    _ => Err(ParseCommandError{}),
                }
            }
            None => Err(ParseCommandError{}),
        }
    }

    pub fn run(&self) -> Result<(), InvalidCommandError> {
        match self {
            Commands::Simulate => {
                simulator::simulate();
                Ok(())
            }
            _ => Err(InvalidCommandError{}),
        }
    }
}
