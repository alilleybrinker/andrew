use anyhow::{Context as _, Result};
use std::env::var;

use crate::cli::PathCommand;

pub fn run(_args: &PathCommand) -> Result<()> {
    // Get, split up, and pretty print the user's path

    let path = var("PATH").context("did not find 'PATH' envvar")?;

    for (i, part) in path.split(":").enumerate() {
        println!("{i:>2}: {part}");
    }

    let mut parts = path.split(":").collect::<Vec<_>>();
    parts.sort();
    let mut dups = vec![];
    for pair in parts.windows(2) {
        let i1 = pair[0];
        let i2 = pair[1];
        if i1 == i2 {
            dups.push(i1);
        }
    }

    for dup in &dups {
        println!("dup found: {dup}");
    }

    Ok(())
}
