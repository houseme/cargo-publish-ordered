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
