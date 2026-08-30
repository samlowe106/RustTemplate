mod cli;

use anyhow::Error;
use clap::Parser;
use cli::Cli;

fn main() -> Result<(), Error> {
    let _cli = Cli::parse();
    Ok(())
}
