use clap::{
    Args,
    Subcommand,
};

#[derive(Args, Debug)]
pub struct TaskGroupArgs {
    #[command(subcommand)]
    command: GroupCommands
}

#[derive(Subcommand, Debug)]
enum GroupCommands {
    Create,
    List,
    Mv,
    Rm
}