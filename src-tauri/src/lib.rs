// Composition root. Declares the clean-architecture module tree, wires the
// dependency-injected AppState, and registers every IPC command in the SINGLE
// `generate_handler!` (a second `invoke_handler` call would silently overwrite).
pub mod commands;
pub mod domain;
pub mod errors;
pub mod infrastructure;
pub mod state;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::new())
        .setup(|_app| {
            // On Windows only: destroy the config-auto-created "main" window and
            // rebuild it with the WebView2 autoplay flag so splash audio plays
            // without requiring a user gesture. Other platforms are unaffected.
            #[cfg(target_os = "windows")]
            {
                use tauri::{Manager, WebviewWindowBuilder};
                if let Some(w) = _app.get_webview_window("main") {
                    w.destroy()?;
                }
                let cfg = _app
                    .config()
                    .app
                    .windows
                    .iter()
                    .find(|wc| wc.label == "main")
                    .cloned()
                    .expect("main window config not found");
                WebviewWindowBuilder::from_config(_app, &cfg)?
                    .additional_browser_args(
                        "--autoplay-policy=no-user-gesture-required",
                    )
                    .build()?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::transaction::get_transactions
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
