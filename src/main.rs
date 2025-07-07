mod cli;

use crate::cli::{Args, Commands};
use cargo_publish_ordered::error::Error;
use cargo_publish_ordered::publisher::Publisher;
use cargo_publish_ordered::workspace::Workspace;
use cargo_publish_ordered::{error, sorter};
use clap::Parser;
use colored::Colorize;
use indicatif::ProgressBar;

fn main() -> Result<(), error::Error> {
    let args = Args::parse();
    match args.command {
        Commands::PublishOrdered(args) => {
            let workspace = Workspace::new(args.manifest_path.as_deref(), &args.exclude)?;
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
                args.dry_run,
                args.no_confirm,
                args.token,
                args.allow_dirty,
                args.verbose,
                args.registry,
            );
            publisher.publish(&packages)?;
        }
        Commands::DepSort(args) => {
            let descending = match args.order.as_str() {
                "asc" => false,
                "desc" => true,
                _ => return Err(Error::InvalidSortOrder(args.order)),
            };
            sorter::sort_dependencies(
                &args.path,
                args.check,
                args.workspace,
                descending,
                args.verbose,
            )?;
        }
    }
    Ok(())
}
