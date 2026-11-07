// Build script for generating completion scripts for the XKCD grabber CLI
use clap::CommandFactory;
use clap_complete::{Shell, generate_to};

#[path = "src/cli.rs"]
mod cli;

fn main() {
    let mut app = cli::XkcdArgs::command();
    let bin_name = app.get_name().to_string();

    let outdir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("completions");
    std::fs::create_dir_all(&outdir).unwrap();
    for shell in [
        Shell::Bash,
        Shell::Zsh,
        Shell::Fish,
        Shell::Elvish,
        Shell::PowerShell,
    ] {
        generate_to(shell, &mut app, &bin_name, &outdir).unwrap();
    }
}
