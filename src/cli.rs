use clap::{Parser, ValueEnum};

/// A utility to grab XKCD comics
#[derive(Parser, Debug)]
pub struct Args {
    /// Set a connection timeout
    #[clap(long, short, default_value = "30")]
    pub timeout: u64,
    /// Print output in a format
    #[clap(long, short, value_enum, default_value = "text")]
    pub output: OutFormat,
    /// The comic to load
    #[clap(long, short)]
    pub num: Option<usize>,
    /// Save image file to current directory
    #[clap(long, short)]
    pub save: bool,
}

#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum OutFormat {
    Json,
    Text,
}
