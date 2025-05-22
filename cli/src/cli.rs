use clap::{
    Parser, 
    Subcommand
};

use crate::commands::{
    TaskGroupArgs,
    TaskArgs,
    TaskPrioritiesArgs,
    DebugArgs,
};

#[derive(Parser, Debug)]
#[command(name = "jarvis", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Task(TaskArgs),
    TaskGroups(TaskGroupArgs),
    TaskPriorities(TaskPrioritiesArgs),
    Debug(DebugArgs),
}