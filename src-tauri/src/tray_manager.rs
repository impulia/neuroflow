use anyhow::Result;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    App, Manager,
};
use tauri_plugin_positioner::{Position, WindowExt};

pub fn setup_tray(app: &App) -> Result<()> {
    let pause_item = MenuItemBuilder::new("Pause Tracking")
        .id("pause")
        .build(app)?;
    let resume_item = MenuItemBuilder::new("Resume Tracking")
        .id("resume")
        .build(app)?;
    let sep1 = PredefinedMenuItem::separator(app)?;
    let reset_item = MenuItemBuilder::new("Reset Today")
        .id("reset_today")
        .build(app)?;
    let sep2 = PredefinedMenuItem::separator(app)?;
    let quit_item = MenuItemBuilder::new("Quit Neflo")
        .id("quit")
        .build(app)?;

    let menu = MenuBuilder::new(app)
        .item(&pause_item)
        .item(&resume_item)
        .item(&sep1)
        .item(&reset_item)
        .item(&sep2)
        .item(&quit_item)
        .build()?;

    let _tray = TrayIconBuilder::new()
        .id("main-tray")
        .tooltip("Neflo")
        .title("●")
        .menu(&menu)
        .menu_on_left_click(false)
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.move_window(Position::TrayCenter);
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        })
        .on_menu_event(|app, event| match event.id().as_ref() {
            "pause" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.eval("window.__neflo_pause && window.__neflo_pause()");
                }
                // Also directly call into state if frontend is not loaded.
                let _ = pause_via_state(app);
            }
            "resume" => {
                let _ = resume_via_state(app);
            }
            "reset_today" => {
                let _ = reset_today_via_state(app);
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}

fn pause_via_state(app: &tauri::AppHandle) -> Result<()> {
    use crate::tracker::Tracker;
    use std::sync::{Arc, Mutex};
    use tauri::Manager;

    if let Some(state) = app.try_state::<Arc<Mutex<Tracker>>>() {
        if let Ok(mut guard) = state.lock() {
            guard.paused = true;
        }
    }
    Ok(())
}

fn resume_via_state(app: &tauri::AppHandle) -> Result<()> {
    use crate::tracker::Tracker;
    use std::sync::{Arc, Mutex};
    use tauri::Manager;

    if let Some(state) = app.try_state::<Arc<Mutex<Tracker>>>() {
        if let Ok(mut guard) = state.lock() {
            guard.paused = false;
            guard.state_start = chrono::Utc::now();
        }
    }
    Ok(())
}

fn reset_today_via_state(app: &tauri::AppHandle) -> Result<()> {
    use crate::tracker::Tracker;
    use chrono::Local;
    use std::sync::{Arc, Mutex};
    use tauri::Manager;

    if let Some(state) = app.try_state::<Arc<Mutex<Tracker>>>() {
        if let Ok(mut guard) = state.lock() {
            let today = Local::now().date_naive();
            guard.db.intervals.retain(|i| {
                let date = i.start.with_timezone(&Local).date_naive();
                date != today
            });
            let _ = guard.storage.save(&guard.db);
        }
    }
    Ok(())
}
