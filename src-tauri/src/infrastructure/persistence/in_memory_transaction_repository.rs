use std::sync::Mutex;

use crate::domain::entities::transaction::Transaction;
use crate::domain::repositories::transaction_repository::TransactionRepository;

/// No-op in-memory implementation of `TransactionRepository`. Stores
/// transactions in a `Vec` behind a `std::sync::Mutex` for interior
/// mutability (future `create` / `update` commands). The mutex lock is
/// acquired, the Vec cloned, and the guard dropped — it is NEVER held
/// across an `.await` point, so `std::sync::Mutex` (not `tokio::sync::Mutex`)
/// is correct here.
#[derive(Default)]
pub struct InMemoryTransactionRepository {
    store: Mutex<Vec<Transaction>>,
}

impl InMemoryTransactionRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TransactionRepository for InMemoryTransactionRepository {
    fn list(&self) -> Result<Vec<Transaction>, String> {
        let guard = self.store.lock().map_err(|e| e.to_string())?;
        Ok(guard.clone())
    }
}
