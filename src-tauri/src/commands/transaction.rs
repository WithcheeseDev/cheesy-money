use crate::domain::entities::transaction::Transaction;
use crate::domain::use_cases::list_transactions::list_transactions;
use crate::errors::AppError;
use crate::state::AppState;

/// Thin IPC handler — parses no args (command takes none), delegates to the
/// `list_transactions` use-case, and maps the domain `String` error into the
/// IPC-safe `AppError::Repository`. Zero business logic here.
#[tauri::command]
pub async fn get_transactions(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<Transaction>, AppError> {
    list_transactions(state.transaction_repo.as_ref()).map_err(AppError::Repository)
}
