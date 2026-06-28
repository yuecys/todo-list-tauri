use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, LogicalSize,
};
use std::fs;
use tauri_plugin_dialog::{DialogExt, FilePath};

#[tauri::command]
fn resize_window(window: tauri::Window, width: f64, height: f64) {
    let _ = window.set_size(LogicalSize::new(width, height));
}

#[tauri::command]
fn hide_window(window: tauri::Window) {
    let _ = window.hide();
}

#[tauri::command]
async fn export_data(app: tauri::AppHandle, data: String) -> Result<bool, String> {
    let file = app
        .dialog()
        .file()
        .add_filter("JSON", &["json"])
        .set_file_name("todo-backup.json")
        .blocking_save_file();
    match file {
        Some(FilePath::Path(p)) => {
            fs::write(&p, data).map_err(|e| e.to_string())?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

#[tauri::command]
async fn import_data(app: tauri::AppHandle) -> Result<String, String> {
    let file = app
        .dialog()
        .file()
        .add_filter("JSON", &["json"])
        .blocking_pick_file();
    match file {
        Some(FilePath::Path(p)) => fs::read_to_string(&p).map_err(|e| e.to_string()),
        _ => Err("cancelled".into()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let window = app.get_webview_window("main").unwrap();

            // Build tray icon using default app icon
            let icon = app.default_window_icon().cloned().unwrap();

            let _tray = TrayIconBuilder::new()
                .icon(icon)
                .tooltip("Todo")
                .on_tray_icon_event({
                    let w = window.clone();
                    move |_tray, event| {
                        if let TrayIconEvent::Click {
                            button: MouseButton::Left,
                            button_state: MouseButtonState::Up,
                            ..
                        } = event
                        {
                            if w.is_visible().unwrap_or(false) {
                                let _ = w.hide();
                            } else {
                                let _ = w.show();
                                let _ = w.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            resize_window,
            hide_window,
            export_data,
            import_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
