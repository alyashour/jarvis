use chrono::NaiveDateTime;
use clap::{
    Args,
    Subcommand,
};

use crate::priorities::Priority;

const DEFAULT_GROUP: &str = "Personal";

#[derive(Args, Debug)]
pub struct TaskArgs {
    #[command(subcommand)]
    command: TaskCommands
}

#[derive(Subcommand, Debug)]
enum TaskCommands {
    /// Creates a new task
    Create {
        /// The name of the new task
        title: String,

        /// When the task is due. Include date and time "DD-MM-YYYY HH:MM".
        #[arg(long, value_name = "DUE DATE")]
        by: Option<NaiveDateTime>,

        /// When you intend to do the task
        #[arg(long, value_name = "DO DATE")]
        r#do: Option<NaiveDateTime>,

        /// The task priority
        #[arg(short, long)]
        priority: Option<Priority>,
        
        /// The group to which this task belongs
        #[arg(short, long, default_value_t = String::from(DEFAULT_GROUP), value_name = "GROUP NAME")]
        group: String,

        /// A parent task id, if applicable
        #[arg(long)]
        parent: Option<i32>,
    },
    /// Lists all available tasks
    List {
        /// Ignore tasks before this date
        #[arg(long, value_name = "BEFORE DATE")]
        before: Option<NaiveDateTime>,

        /// Ignore tasks after this date
        #[arg(long, value_name = "AFTER DATE")]
        after: Option<NaiveDateTime>,

        /// Filter based on some key=value pair
        #[arg(long)]
        filter: Option<String>, // Not yet implemented.

        /// Include only tasks in this group
        #[arg(long, value_name = "GROUP NAME")]
        group: Option<String>,
    },
    /// Updates a task
    Update {
        /// Task ID (e.g., abc)
        #[arg(value_name = "TASK ID")]
        id: String,

        /// The update to be made key=value
        #[arg(value_name = "UPDATE")]
        set: String,
    },
    /// Completes a task, progress = 100%
    Complete {
        /// Task ID (e.g., abc)
        #[arg(value_name = "TASK ID")]
        id: String,
    },
    /// Deletes a task, moving it to the trash group
    Rm {
        /// Task ID (e.g., abc)
        #[arg(value_name = "TASK ID")]
        id: String,

        /// Immediately deletes it without sending to trash group
        #[arg(short = 'I', long)]
        immediate: bool,
    },
}