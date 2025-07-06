mod cli;
mod error;
mod publisher;
mod workspace;

use crate::cli::Cli;
use crate::publisher::Publisher;
use crate::workspace::Workspace;
use clap::Parser;
use colored::Colorize;

fn main() -> Result<(), error::Error> {
    let cli = Cli::parse();

    // Analyze the workspace
    let workspace = Workspace::new(None, &cli.exclude)?;
    let packages = workspace.packages_to_publish();

    if packages.is_empty() {
        println!("{}", "No need to publish crate".yellow());
        return Ok(());
    }

    // Execute release
    let publisher = Publisher::new(cli.dry_run, cli.no_confirm, cli.token, cli.allow_dirty);
    publisher.publish(&packages)?;

    Ok(())
}
