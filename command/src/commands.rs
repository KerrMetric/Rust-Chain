use crate::simulator;
use accounts::account;
use db;
use rust_chain_error::invalid_command_error::InvalidCommandError;
use rust_chain_error::parse_command_error::ParseCommandError;
use std::collections::VecDeque;

pub enum Commands {
    CreateAccount,
    Simulate(String, String),
}

impl Commands {
    pub fn parse(mut args: VecDeque<String>) -> Result<Commands, ParseCommandError> {
        match args.pop_front() {
            Some(v) => match v.as_str() {
                "create_account" => Ok(Commands::CreateAccount),
                "simulate" => {
                    let from = args
                        .pop_front()
                        .expect("invalid argument. need to from address");
                    let to = args
                        .pop_front()
                        .expect("invalid argument. need to to address");
                    Ok(Commands::Simulate(from, to))
                }
                _ => Err(ParseCommandError {}),
            },
            None => Err(ParseCommandError {}),
        }
    }

    pub fn run(&self) -> Result<(), InvalidCommandError> {
        match self {
            Commands::Simulate(from, to) => {
                simulator::simulate(from, to);
                Ok(())
            }
            Commands::CreateAccount => {
                let account = account::Account::new();
                dbg!(&account);
                let _ = db::account_db::set(&account)
                    .map_err(|e| panic!("failed to create account because {}", e));
                Ok(())
            }
        }
    }
}
