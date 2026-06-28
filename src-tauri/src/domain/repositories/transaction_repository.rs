use crate::domain::entities::transaction::Transaction;

/// Dependency-inversion seam between `domain/` and `infrastructure/`.
///
/// The trait is intentionally **sync** so `Arc<dyn TransactionRepository>`
/// remains object-safe on stable Rust (async fn in traits is not yet
/// dyn-compatible without `async-trait`). The lock inside the concrete
/// implementation is never held across an `.await`, satisfying the
/// `std::sync::Mutex` safety rule.
pub trait TransactionRepository {
    fn list(&self) -> Result<Vec<Transaction>, String>;
}
