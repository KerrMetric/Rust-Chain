use command::commands;

fn main() {
    println!("Start Rust Chain!");

    let args: Vec<String> = std::env::args().collect();

    let command = match commands::Commands::parse(args.get(1)) {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = command.run() {
        println!("{}", e);
    }

    println!("completed!");
}
