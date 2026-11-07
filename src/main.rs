mod cli;
mod client;

use anyhow::Ok;
use anyhow::Result;
use clap::Parser;

use crate::cli::XkcdArgs;
use crate::client::XkcdClient;

fn main() -> Result<()> {
    let args = XkcdArgs::parse();
    let client = XkcdClient::new(args);
    client.run()?;
    Ok(())
}
