mod cli;
mod client;

use anyhow::Ok;
use anyhow::Result;
use clap::Parser;

use crate::cli::XkcdArgs;
use crate::client::XkcdClient;

fn main() -> Result<()> {
    let args = XkcdArgs::parse();
    if let Some(shell) = args.completions {
        shell.generate();
        return Ok(());
    }

    let client = XkcdClient::new(args);
    client.run()?;
    Ok(())
}
