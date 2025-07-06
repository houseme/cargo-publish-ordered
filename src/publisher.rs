use crate::error::Error;
use colored::Colorize;
use dialoguer::{Confirm, theme::ColorfulTheme};
use indicatif::{ProgressBar, ProgressStyle};
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
        println!("{}", "Release order:".bold().green());
        for pkg in packages {
            println!("  - {} v{}", pkg.name, pkg.version);
        }

        if !self.no_confirm && !self.dry_run {
            let confirmed = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Confirm to release the above crate?")
                .interact()?;
            if !confirmed {
                return Err(Error::UserCancelled);
            }
        }

        let pb = ProgressBar::new(packages.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}",
                )
                .unwrap(),
        );

        for pkg in packages {
            pb.set_message(format!("release {} v{}", pkg.name, pkg.version));
            let mut cmd = Command::new("cargo");
            cmd.arg("publish")
                .current_dir(pkg.manifest_path.parent().unwrap());

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
                pb.inc(1);
                println!(
                    "{} {} v{} {}",
                    "Successfully released".bold().green(),
                    pkg.name,
                    pkg.version,
                    if self.dry_run { "(Dry running)" } else { "" }
                );
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                pb.abandon_with_message(format!("Publish {} failed", pkg.name));
                return Err(Error::Publish(stderr.to_string()));
            }
        }

        pb.finish_with_message("Release completed");
        Ok(())
    }
}
