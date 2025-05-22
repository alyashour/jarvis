use clap::{
    Args,
    Subcommand,
};

#[derive(Args, Debug)]
pub struct DebugArgs {
    #[command(subcommand)]
    command: DebugCommands
}

#[derive(Subcommand, Debug)]
enum DebugCommands {
    Db {
        #[command(subcommand)]
        command: DbCommands
    }
}

#[derive(Subcommand, Debug)]
enum DbCommands {
    Clear,
    Seed,
    Reset,
    TestConnection
}