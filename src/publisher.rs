use crate::error::Error;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm};
use std::process::Command;

pub struct Publisher {
    dry_run: bool,
    no_confirm: bool,
    token: Option<String>,
    allow_dirty: bool,
}

impl Publisher {
    pub fn new(dry_run: bool, no_confirm: bool, token: Option<String>, allow_dirty: bool) -> Self {
        Publisher {
            dry_run,
            no_confirm,
            token,
            allow_dirty,
        }
    }

    pub fn publish(&self, packages: &[&cargo_metadata::Package]) -> Result<(), Error> {
        // Show publishing order
        println!("{}", "Release order:".bold().green());
        for pkg in packages {
            println!("  - {} v{}", pkg.name, pkg.version);
        }

        // Interactive confirmation
        if !self.no_confirm && !self.dry_run {
            let confirmed = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Confirm to release the above crate?")
                .interact()?;
            if !confirmed {
                return Err(Error::UserCancelled);
            }
        }

        // Execute release
        for pkg in packages {
            println!(
                "{} {} v{}...",
                "Release in progress".bold().yellow(),
                pkg.name,
                pkg.version
            );

            let mut cmd = Command::new("cargo");
            cmd.arg("publish")
                .current_dir(&pkg.manifest_path.parent().unwrap());

            if self.dry_run {
                cmd.arg("--dry-run");
            }
            if self.allow_dirty {
                cmd.arg("--allow-dirty");
            }
            if let Some(token) = &self.token {
                cmd.arg("--token").arg(token);
            }

            let output = cmd.output()?;
            if output.status.success() {
                println!(
                    "{} {} v{} {}",
                    "Successfully released".bold().green(),
                    pkg.name,
                    pkg.version,
                    if self.dry_run { "(Dry running)" } else { "" }
                );
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(Error::Publish(stderr.to_string()));
            }
        }

        Ok(())
    }
}
