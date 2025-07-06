use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "cargo-publish-ordered",
    about = "Publish crates in workspaces in dependency order",
    version
)]
pub struct Cli {
    #[arg(long, help = "Simulate the release process, not actually executed")]
    pub dry_run: bool,
    #[arg(long, help = "Skip interactive confirmation")]
    pub no_confirm: bool,
    #[arg(long, help = "crates.io API token")]
    pub token: Option<String>,
    #[arg(long, help = "Allow uncommitted changes")]
    pub allow_dirty: bool,
    #[arg(long, help = "Exclude specific crate not to publish")]
    pub exclude: Vec<String>,
}
