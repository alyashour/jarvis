use clap::{
    Args,
    Subcommand,
};

#[derive(Args, Debug)]
pub struct TaskArgs {
    #[command(subcommand)]
    command: TaskCommands
}

#[derive(Subcommand, Debug)]
enum TaskCommands {
    Create,
    List,
    Update,
    Complete,
    Rm,
}