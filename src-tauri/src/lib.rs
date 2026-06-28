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
        .invoke_handler(tauri::generate_handler![
            commands::transaction::get_transactions
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
