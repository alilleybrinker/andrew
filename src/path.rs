use crate::cli::PathCommand;
use anyhow::{Context as _, Result};
use std::env::var;

/// Get, split up, and pretty print the user's path.
pub fn run(_args: &PathCommand) -> Result<()> {
    let path = var("PATH").context("did not find 'PATH' envvar")?;

    for (i, part) in path.split(":").enumerate() {
        let i = i + 1;
        println!("{i:>2}: {part}");
    }

    Ok(())
}
