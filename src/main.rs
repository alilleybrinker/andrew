use std::process::ExitCode;

use anyhow::Result;
use clap::Parser as _;

mod cli;
mod path;

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn run() -> Result<()> {
    let args = cli::Args::parse();

    match &args.subcommand {
        cli::Command::Path(args) => path::run(&args)?,
    }

    Ok(())
}
