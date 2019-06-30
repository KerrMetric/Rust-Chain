use simulator::simulate;

enum Command {
    CreateAccount,
    Simulate,
}

impl Command {
    fn parse(arg: Option<&String>) -> Option<Command> {
        match arg {
            Some(v) => {
                let s = v.as_str();
                match s {
                    "create_account" => Some(Command::CreateAccount),
                    "simulate" => Some(Command::Simulate),
                    _ => None,
                }
            }
            None => None,
        }
    }
}

fn main() {
    println!("Start Rust Chain!");

    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    let command = match Command::parse(args.get(1)) {
        Some(c) => c,
        None => {
            println!("command is invalid!");
            std::process::exit(1);
        }
    };

    match command {
        Command::Simulate => simulate(),
        _ => println!("not implemented"),
    }

    println!("completed!");
}
