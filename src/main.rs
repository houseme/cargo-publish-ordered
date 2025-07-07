mod cli;

use crate::cli::Cli;
use cargo_publish_ordered::error;
use cargo_publish_ordered::publisher::Publisher;
use cargo_publish_ordered::workspace::Workspace;
use clap::Parser;
use colored::Colorize;
use indicatif::ProgressBar;

fn main() -> Result<(), error::Error> {
    let cli = Cli::parse();

    let workspace = Workspace::new(cli.manifest_path.as_deref(), &cli.exclude)?;
    let packages = workspace.packages_to_publish();

    if packages.is_empty() {
        let pb = ProgressBar::new(0);
        pb.println(format!(
            "{}",
            "There is no crate that needs to be published".yellow()
        ));
        return Ok(());
    }

    let publisher = Publisher::new(
        cli.dry_run,
        cli.no_confirm,
        cli.token,
        cli.allow_dirty,
        cli.verbose,
        cli.registry,
    );
    publisher.publish(&packages)?;

    Ok(())
}
