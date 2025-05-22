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
    /// Creates a new task priority
    Create { 
        /// Name of the new priority
        priority_name: String 
    },
    /// View all current priorities
    #[command(aliases = vec!("view", "ls"))]
    List { 
        /// Prints comma separated titles only
        #[arg(long)]
        plain: bool
    },
    /// Reorder the current priorities
    Reorder { 
        /// The new ordering, highest to lowest
        #[arg(long)]
        ids: Vec<i32>
    },
    /// Delete a priority
    #[command(alias = "rm")]
    Delete {
        /// Name of the priority to be deleted
        #[arg(value_name = "PRIORITY CLASS")]
        priority: String,

        /// Should the priority's tasks be deleted?
        #[arg(short='d', long="delete", value_name="DELETE", required_unless_present = "cascade_reassign", conflicts_with = "cascade_reassign")]
        cascade_delete: Option<bool>,

        /// Reassign the priority's tasks to another priority
        #[arg(short='r', long="reassign", value_name="GROUP NAME", required_unless_present = "cascade_delete", conflicts_with = "cascade_delete")]
        cascade_reassign: Option<String>,
    },
}