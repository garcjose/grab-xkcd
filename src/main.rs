mod cli;

use crate::cli::GrabXkcd;
use clap::Parser;

fn main() {
    let args = GrabXkcd::parse();
    // ... TODO
}
