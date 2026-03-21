pub mod commands;
pub mod config;
pub mod models;
pub mod stats;
pub mod storage;
pub mod system;
pub mod tracker;
pub mod tray_manager;
pub mod utils;

use crate::commands::emit_update;
use crate::config::load_config;
use crate::storage::Storage;
use crate::system::get_idle_time;
use crate::tracker::Tracker;
use std::sync::{Arc, Mutex};
use std::time::Duration;
pub fn run() {
    // Acquire single-instance lock before building Tauri app.
    // We keep the lock guard alive for the lifetime of the process by leaking it.
    let lock_path = {
        let mut p = dirs::home_dir().expect("Could not find home directory");
        p.push(".neflo");
        if !p.exists() {
            std::fs::create_dir_all(&p).expect("Could not create ~/.neflo");
        }
        p.push("neflo.lock");
        p
    };
    let lock_file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)
        .expect("Could not open lock file");
    let mut lock = fd_lock::RwLock::new(lock_file);
    let _guard = match lock.try_write() {
        Ok(g) => g,
        Err(_) => {
            eprintln!("Neflo is already running.");
            std::process::exit(1);
        }
    };

    let config = load_config().unwrap_or_default();
    let storage = Storage::new().expect("Could not create storage");
    let tracker = Tracker::new(
        storage,
        config.default_threshold_mins,
        config.duration.clone(),
    )
    .expect("Could not create tracker");

    let tracker_state: Arc<Mutex<Tracker>> = Arc::new(Mutex::new(tracker));
    let config_state: Arc<Mutex<config::Config>> = Arc::new(Mutex::new(config));

    let tracker_for_thread = Arc::clone(&tracker_state);
    let config_for_thread = Arc::clone(&config_state);

    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .manage(tracker_state)
        .manage(config_state)
        .invoke_handler(tauri::generate_handler![
            commands::get_current_state,
            commands::get_stats,
            commands::get_weekly_chart_data,
            commands::get_config,
            commands::update_config,
            commands::pause_tracking,
            commands::resume_tracking,
            commands::reset_today,
        ])
        .setup(move |app| {
            // Create the main hidden webview window.
            let window =
                tauri::WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::App("/".into()))
                    .title("Neflo")
                    .inner_size(340.0, 480.0)
                    .decorations(false)
                    .visible(false)
                    .transparent(true)
                    .resizable(false)
                    .skip_taskbar(true)
                    .build()?;

            // Position near the tray on first show.
            use tauri_plugin_positioner::{Position, WindowExt};
            let _ = window.move_window(Position::TrayCenter);

            // Set up the tray icon and menu.
            tray_manager::setup_tray(app)?;

            // Spawn the background tracking thread.
            // It ticks the tracker every second and emits a `tracker-update`
            // event so the frontend receives push updates instead of polling.
            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                loop {
                    std::thread::sleep(Duration::from_secs(1));
                    let idle = get_idle_time();
                    let now = chrono::Utc::now();

                    let mut guard = match tracker_for_thread.lock() {
                        Ok(g) => g,
                        Err(_) => continue,
                    };

                    if guard.paused {
                        // Still emit so the UI stays in sync (e.g. shows paused state).
                        if let Ok(cfg) = config_for_thread.lock() {
                            emit_update(&app_handle, &guard, &cfg);
                        }
                        continue;
                    }

                    if guard.should_stop(now) {
                        if !guard.session_ended_saved {
                            let _ = guard.storage.save(&guard.db);
                            guard.session_ended_saved = true;
                        }
                        if let Ok(cfg) = config_for_thread.lock() {
                            emit_update(&app_handle, &guard, &cfg);
                        }
                        continue;
                    }

                    let _ = guard.tick(idle, now);

                    // Emit the full update to the frontend.
                    if let Ok(cfg) = config_for_thread.lock() {
                        emit_update(&app_handle, &guard, &cfg);
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Neflo");
}
