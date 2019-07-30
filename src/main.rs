use command::commands;
use std::collections::VecDeque;

fn main() {
    println!("Start Rust Chain!");

    let mut args = std::env::args().collect::<VecDeque<String>>();
    let _ = args.pop_front();

    let command = commands::Commands::parse(args).unwrap_or_else(|arg| panic!("{}", arg));

    command
        .run()
        .unwrap_or_else(|e| panic!("failed to run. because {}", e));

    println!("completed!");
}
