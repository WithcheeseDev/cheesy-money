use crate::domain::entities::transaction::Transaction;
use crate::domain::repositories::transaction_repository::TransactionRepository;

/// Delegates to the repository trait. Business logic for listing transactions
/// lives here; `commands/` only calls this and maps errors into `AppError`.
pub fn list_transactions(repo: &dyn TransactionRepository) -> Result<Vec<Transaction>, String> {
    repo.list()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct StubRepo;

    impl TransactionRepository for StubRepo {
        fn list(&self) -> Result<Vec<Transaction>, String> {
            Ok(vec![])
        }
    }

    #[test]
    fn list_transactions_returns_empty_for_empty_repo() {
        // SC2: given empty repo, when list_transactions called, then Ok([])
        assert_eq!(list_transactions(&StubRepo).unwrap(), vec![]);
    }
}
