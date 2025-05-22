use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Priority {
    Low,
    #[clap(alias = "med")]
    Medium,
    High
}