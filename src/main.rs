mod config;
mod models;
mod report;
mod stats;
mod storage;
mod system;
mod tracker;
mod tui;
mod update;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};
use fd_lock::RwLock;
use report::Reporter;
use std::fs::OpenOptions;
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
        /// Start time (HH:MM)
        #[arg(long)]
        start_time: Option<String>,
        /// End time (HH:MM)
        #[arg(long)]
        end_time: Option<String>,
        /// Timeout duration (e.g. 8h, 30m)
        #[arg(short, long)]
        timeout: Option<String>,
    },
    /// Generate a report of focus/idle time
    Report,
    /// Update neflo to the latest version
    SelfUpdate,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = config::load_config()?;
    let storage = Storage::new()?;

    match cli.command {
        Commands::Start {
            threshold,
            start_time,
            end_time,
            timeout,
        } => {
            let base_dir = Storage::get_base_dir()?;
            let lock_path = base_dir.join("neflo.lock");
            let lock_file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(lock_path)?;

            let mut lock = RwLock::new(lock_file);
            let _guard = lock.try_write().map_err(|_| {
                anyhow::anyhow!("Another instance of Neflo is already running. Please close it before starting a new one.")
            })?;

            let threshold = threshold.unwrap_or(config.default_threshold_mins);
            let start_time = start_time.or(config.start_time);
            let end_time = end_time.or(config.end_time);
            let timeout = timeout.or(config.timeout);

            let mut tracker = Tracker::new(storage.clone(), threshold, start_time, end_time, timeout)?;

            tui::run_tui(&mut tracker)?;

            // Final save
            tracker.storage.save(&tracker.db)?;

            // Report
            println!("\nSession ended automatically or by user.");
            let reporter = Reporter::new(storage);
            reporter.report()?;
        }
        Commands::Report => {
            let reporter = Reporter::new(storage);
            reporter.report()?;
        }
        Commands::SelfUpdate => {
            update::update()?;
        }
    }

    Ok(())
}
