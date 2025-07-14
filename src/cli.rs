use clap::{Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(
    name = "publish-ordered",
    about = "A tool to publish crates in workspaces in dependency order and sort dependencies.",
    version
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Publish packages in the workspace in order
    PublishOrdered(PublishOrderedArgs),
    /// Sort dependencies in Cargo.toml alphabetically
    DepSort(SortArgs),
}

#[derive(Parser, Debug)]
pub struct PublishOrderedArgs {
    #[arg(long, help = "Simulate the release process, not actually executed")]
    pub dry_run: bool,
    #[arg(long, help = "Skip interactive confirmation")]
    pub no_confirm: bool,
    #[arg(long, help = "crates.io API token", env = "CARGO_REGISTRY_TOKEN")]
    pub token: Option<String>,
    #[arg(long, help = "Allow uncommitted changes")]
    pub allow_dirty: bool,
    #[arg(long, help = "Exclude specific crate not to publish")]
    pub exclude: Vec<String>,
    #[arg(long, help = "Show detailed logs")]
    pub verbose: bool,
    #[arg(long, help = "Specify the workspace Cargo.toml path")]
    pub manifest_path: Option<String>,
    #[arg(
        long,
        help = "Specify the publishing registry, default: crates-io",
        default_value = "crates-io"
    )]
    pub registry: String,
}

#[derive(Parser, Debug)]
pub struct SortArgs {
    /// Path to the Cargo.toml file to sort
    #[arg(long, default_value = "Cargo.toml")]
    pub path: String,
    /// Check if the dependencies are sorted without writing changes
    #[arg(long)]
    pub check: bool,
    /// Also sort the [workspace.dependencies] table
    #[arg(long)]
    pub workspace: bool,
    /// Sort order: 'asc' or 'desc'
    #[arg(long, default_value = "asc")]
    pub order: String,
    /// Show detailed logs
    #[arg(long, short)]
    pub verbose: bool,
}
