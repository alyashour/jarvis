mod cli;
mod commands;
mod priorities;

use clap::Parser;

use cli::Cli;

fn main() {
    let app = Cli::parse();

    match app.command {
        cli::Commands::Task(args) => args.run(),
        _ => println!("Command not yet implemented.")
    }
}
