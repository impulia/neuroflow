mod config;
mod models;
mod report;
mod storage;
mod system;
mod tracker;
mod config;
mod tui;

use anyhow::Result;
use clap::{Parser, Subcommand};
use report::Reporter;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use storage::Storage;
use tracker::Tracker;

#[derive(Parser)]
#[command(name = "neflo")]
#[command(about = "A simple focus and idle time tracker for macOS", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start tracking focus/idle time
    Start {
        /// Idle threshold in minutes
        #[arg(short, long)]
        threshold: Option<u64>,
    },
    /// Generate a report of focus/idle time
    Report {
        /// Show report in Terminal UI mode
        #[arg(long)]
        tui: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = config::load_config()?;
    let storage = Storage::new()?;

    match cli.command {
        Commands::Start { threshold } => {
            let threshold = threshold.unwrap_or(config.default_threshold_mins);
            let tracker = Tracker::new(storage, threshold);

            let running = Arc::new(AtomicBool::new(true));
            let r = running.clone();

            ctrlc::set_handler(move || {
                r.store(false, Ordering::SeqCst);
            })
            .expect("Error setting Ctrl-C handler");

            tracker.start(running)?;
        }
        Commands::Report { tui } => {
            let reporter = Reporter::new(storage);
            if tui {
                tui::show_tui(reporter)?;
            } else {
                reporter.report()?;
            }
        }
    }

    Ok(())
}
