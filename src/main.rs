mod cli;
mod client;

use anyhow::Ok;
use anyhow::Result;
use clap::Parser;

use crate::cli::Args;
use crate::client::XkcdClient;

fn main() -> Result<()> {
    let args = Args::parse();
    let client = XkcdClient::new(args);
    client.run()?;
    Ok(())
}
