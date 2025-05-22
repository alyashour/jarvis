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

/// Database debug commands, see subcommands for more details
#[derive(Subcommand, Debug)]
enum DbCommands {
    /// Clears the tables in the database
    Clear,
    /// Drops the database (deletes tables)
    Drop,
    /// Seeds the tables with default values
    Seed,
    /// Calls clear and seed
    Reset,
    /// Tests the connection with the database
    TestConnection
}