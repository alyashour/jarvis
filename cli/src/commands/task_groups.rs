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
    /// Creates a new group
    #[command(alias = "new")]
    Create {
        /// Create a brand new group. Lead with '.' to hide
        #[arg(long)]
        name: String,
    },
    /// Lists all groups
    #[command(alias = "ls")]
    List {
        /// Show hidden groups
        #[arg(long)]
        hidden: bool,
    },
    /// Renames a group
    #[command(alias = "rn")]
    Rename {
        /// The old group's name
        group_name: String,

        /// The new name
        new_group_name: String,
    },
    /// Deletes a group
    #[command(alias = "rm")]
    Delete {
        /// The group's name
        group_name: String,

        /// Should the group's tasks be deleted?
        #[arg(short='d', long="delete")]
        cascade_delete: bool,

        /// Reassign the group's tasks to another group
        #[arg(short='r', long="reassign", value_name="GROUP NAME")]
        cascade_reassign: String,
    },
}