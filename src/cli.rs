#[derive(Debug, clap::Parser)]
#[command(version, about)]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: Command,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    /// Pretty print the user's PATH
    Path(PathCommand),
}

#[derive(Debug, clap::Args)]
pub struct PathCommand {}
