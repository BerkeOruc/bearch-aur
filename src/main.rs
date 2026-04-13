use clap::Parser;
use std::process;

mod cli;
mod aur;
mod alpm;

#[derive(Parser, Debug)]
#[command(name = "bearch-aur")]
#[command(about = "BEAUR - Rust AUR Client for Arch Linux", long_about = None)]
#[command(version = "0.1.0")]
struct Args {
    #[command(subcommand)]
    command: cli::Command,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let args = Args::parse();

    if let Err(e) = cli::run(args.command) {
        tracing::error!("Error: {}", e);
        process::exit(1);
    }
}