use std::sync::Arc;

use crate::domain::repositories::transaction_repository::TransactionRepository;
use crate::infrastructure::persistence::in_memory_transaction_repository::InMemoryTransactionRepository;

/// Application state registered once via `.manage(AppState::new())` in
/// `lib.rs`. Tauri wraps it in an `Arc` internally; the `Arc` inside holds
/// the repository behind the trait interface for dependency injection.
///
/// Note: `Mutex` lives INSIDE `InMemoryTransactionRepository.store`, not on
/// `AppState` — `Arc<dyn Trait + Send + Sync>` is already shareable without
/// an outer lock. This avoids the pitfall of holding `std::sync::Mutex`
/// across an `.await` boundary.
pub struct AppState {
    pub transaction_repo: Arc<dyn TransactionRepository + Send + Sync>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            transaction_repo: Arc::new(InMemoryTransactionRepository::new()),
        }
    }
}
