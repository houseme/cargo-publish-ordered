use crate::error::Error;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

pub struct Publisher {
    dry_run: bool,
    no_confirm: bool,
    token: Option<String>,
    allow_dirty: bool,
    verbose: bool,
    registry: String,
}

impl Publisher {
    pub fn new(
        dry_run: bool,
        no_confirm: bool,
        token: Option<String>,
        allow_dirty: bool,
        verbose: bool,
        registry: String,
    ) -> Self {
        Publisher {
            dry_run,
            no_confirm,
            token,
            allow_dirty,
            verbose,
            registry,
        }
    }

    pub fn publish(&self, packages: &[&cargo_metadata::Package]) -> Result<(), Error> {
        let pb = ProgressBar::new(packages.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}",
                )
                .unwrap(),
        );

        pb.println(format!("{}", "Release order:".bold().green()));
        for pkg in packages {
            pb.println(format!("  - {} v{}", pkg.name, pkg.version));
        }

        if !self.no_confirm && !self.dry_run {
            let confirmed = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Confirm to release the above crate?")
                .interact()?;
            if !confirmed {
                return Err(Error::UserCancelled);
            }
        }

        let success_count = AtomicUsize::new(0);
        let results = packages
            .par_iter()
            .map(|&pkg| {
                pb.set_message(format!("release {} v{}", pkg.name, pkg.version));
                let mut cmd = Command::new("cargo");
                cmd.arg("publish")
                    .current_dir(pkg.manifest_path.parent().unwrap())
                    .arg("--registry")
                    .arg(&self.registry);

                if self.dry_run {
                    cmd.arg("--dry-run");
                }
                if self.allow_dirty {
                    cmd.arg("--allow-dirty");
                }
                if let Some(token) = &self.token {
                    cmd.arg("--token").arg(token);
                }

                if self.verbose {
                    pb.println(format!("Execute the command:{cmd:?}"));
                }

                let output = cmd.output()?;
                if output.status.success() {
                    success_count.fetch_add(1, Ordering::SeqCst);
                    pb.inc(1);
                    pb.println(format!(
                        "{} {} v{} {}",
                        "Successfully released".bold().green(),
                        pkg.name,
                        pkg.version,
                        if self.dry_run { "(Dry running)" } else { "" }
                    ));
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    pb.abandon_with_message(format!("Publish {} failed", pkg.name));
                    Err(Error::Publish(
                        pkg.name.clone().to_string(),
                        stderr.to_string(),
                    ))
                }
            })
            .collect::<Vec<_>>();

        pb.finish_with_message("Release completed");

        for result in results {
            result?;
        }

        if !self.dry_run && success_count.load(Ordering::SeqCst) > 0 {
            pb.println(format!("{}", "Wait for crates.io rate limit...".yellow()));
            std::thread::sleep(Duration::from_secs(5));
        }

        Ok(())
    }
}
