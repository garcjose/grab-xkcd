use clap::{Parser, ValueEnum};

/// A utility to grab XKCD comics
#[derive(Parser, Debug)]
pub struct XkcdArgs {
    /// Set a connection timeout
    #[arg(long, short, default_value = "30")]
    pub timeout: u64,
    /// Print output in a format
    #[arg(long, short, value_enum, default_value = "text")]
    pub output: OutFormat,
    /// The comic to load
    #[arg(long, short)]
    pub num: Option<usize>,
    /// Save image file to current directory
    #[arg(long, short)]
    pub save: bool,
}

#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum OutFormat {
    Json,
    Text,
}
