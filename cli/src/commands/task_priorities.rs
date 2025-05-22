use clap::{
    Args,
    Subcommand,
};

#[derive(Args, Debug)]
pub struct TaskPrioritiesArgs {
    #[command(subcommand)]
    command: TaskPrioritiesCommands
}

#[derive(Subcommand, Debug)]
enum TaskPrioritiesCommands {
    Create,
    List,
    Mv,
    Reorder,
    Rm,
}