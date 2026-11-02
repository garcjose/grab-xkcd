mod cli;

use anyhow::Ok;
use anyhow::Result;
use clap::Parser;

use crate::cli::Args;
use crate::cli::XkcdClient;

fn main() -> Result<()> {
    let args = Args::parse();
    let client = XkcdClient::new(args);
    //client.run()
    Ok(())
}
