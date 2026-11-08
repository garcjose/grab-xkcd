use clap::{CommandFactory, Parser, ValueEnum};
/// A utility to grab XKCD comics
#[derive(Parser, Debug)]
pub struct XkcdArgs {
    /// Set a connection timeout
    #[arg(long, short, default_value_t = 30)]
    pub timeout: u64,
    /// Print output in a format
    #[arg(long, short, value_enum, default_value_t = OutFormat::Text)]
    pub output: OutFormat,
    /// The comic to load
    #[arg(long, short)]
    pub num: Option<usize>,
    /// Save image file to current directory
    #[arg(long, short)]
    pub save: bool,
    /// Generate a SHELL completion script and print to stdout
    #[arg(long, short, value_enum, value_name = "SHELL")]
    pub completions: Option<Shell>,
}

#[derive(Debug, ValueEnum, Copy, Clone)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}

impl From<Shell> for clap_complete::Shell {
    fn from(shell: Shell) -> Self {
        match shell {
            Shell::Bash => clap_complete::Shell::Bash,
            Shell::Zsh => clap_complete::Shell::Zsh,
            Shell::Fish => clap_complete::Shell::Fish,
            Shell::PowerShell => clap_complete::Shell::PowerShell,
            Shell::Elvish => clap_complete::Shell::Elvish,
        }
    }
}

impl Shell {
    pub fn generate(&self) {
        let mut app = XkcdArgs::command();
        let bin_name = app.get_name().to_string();
        // Dereference *self to pass an owned value into .into()
        let shell: clap_complete::Shell = (*self).into();
        clap_complete::generate(shell, &mut app, bin_name, &mut std::io::stdout());
    }
}

#[derive(ValueEnum, Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum OutFormat {
    Json,
    #[default]
    Text,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_cli() {
        let mut cmd = XkcdArgs::command();
        println!("{:?}", cmd.render_usage());
        cmd.debug_assert();
    }
}
