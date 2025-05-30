mod cli;
mod commands;
mod priorities;

use clap::Parser;

use cli::Cli;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        _ => println!("Command not yet implemented.")
    }
}
