use crate::aur::AurClient;
use crate::alpm;
use clap::Parser;

#[derive(Parser, Debug)]
pub enum Command {
    /// Search for packages in AUR
    Search {
        /// Search query
        query: String,
    },
    /// Install a package from AUR
    Install {
        /// Package name(s) to install
        packages: Vec<String>,
    },
    /// Remove installed AUR packages
    Remove {
        /// Package name(s) to remove
        packages: Vec<String>,
    },
    /// Update all AUR packages
    Update,
    /// Show information about a package
    Info {
        /// Package name
        package: String,
    },
}

pub fn run(command: Command) -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async { run_async(command).await })
}

async fn run_async(command: Command) -> Result<(), Box<dyn std::error::Error>> {
    let client = AurClient::new();

    match command {
        Command::Search { query } => {
            let results = client.search(&query).await?;
            if results.is_empty() {
                println!("No packages found for '{}'", query);
                return Ok(());
            }
            println!("Search results for '{}':", query);
            println!("{}", "-".repeat(60));
            for pkg in results {
                println!("{} - {}", pkg.name, pkg.version);
                println!("  {}", pkg.description);
                println!();
            }
        }
        Command::Install { packages } => {
            if packages.is_empty() {
                return Err("No packages specified".into());
            }
            for pkg in &packages {
                println!("Installing {}...", pkg);
                let info = client.info(pkg).await?;
                println!(
                    "  Package: {} ({})",
                    info.name, info.version
                );
                println!("  Description: {}", info.description);
                println!("  Maintainer: {}", info.maintainer.unwrap_or_else(|| "orphan".to_string()));
                println!("  URL: {}", info.url.as_deref().unwrap_or("N/A"));
                println!("  License: {}", info.license.join(", "));
                println!();
                println!("Note: Actual installation requires libalpm integration.");
                println!("This is a placeholder - actual install not implemented yet.");
            }
        }
        Command::Remove { packages } => {
            if packages.is_empty() {
                return Err("No packages specified".into());
            }
            for pkg in &packages {
                println!("Removing {}...", pkg);
                println!("Note: This requires libalpm integration to actually remove packages.");
            }
        }
        Command::Update => {
            println!("Updating all AUR packages...");
            println!("Note: This requires libalpm integration to work properly.");
            if let Err(e) = alpm::check_updates().await {
                println!("Warning: Could not check for updates: {}", e);
            }
        }
        Command::Info { package } => {
            let info = client.info(&package).await?;
            println!("Package: {}", info.name);
            println!("Version: {}", info.version);
            println!("Description: {}", info.description);
            println!("Maintainer: {}", info.maintainer.unwrap_or_else(|| "orphan".to_string()));
            println!("URL: {}", info.url.as_deref().unwrap_or("N/A"));
            println!("License: {}", info.license.join(", "));
            println!("Votes: {}", info.num_votes);
            println!("Popularity: {:.2}", info.popularity);
            println!("First Submitted: {}", info.firstsubmitted);
            println!("Last Updated: {}", info.lastmodified);
        }
    }

    Ok(())
}