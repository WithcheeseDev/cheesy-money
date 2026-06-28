// Tier 1 — Additional domain unit tests for list_transactions use-case.
//
// rust-dev owns the ONE inline #[cfg(test)] test in
// domain/use_cases/list_transactions.rs (empty repo → Ok(vec![])).
// This file adds distinct contract-verification tests using stubs that
// return NON-empty results and errors — proving list_transactions passes
// data through faithfully, not just returning a hardcoded empty vec.
//
// COMPILATION PREREQUISITE: lib.rs must declare:
//   pub mod commands; pub mod domain; pub mod errors;
//   pub mod infrastructure; pub mod state;
// Currently lib.rs is scaffold-only (greet + run). These tests will not
// compile until the module declarations are added (lead-owned file).
//
// Run from src-tauri/: cargo test --test domain_list_transactions

use cheesy_money_lib::domain::entities::transaction::Transaction;
use cheesy_money_lib::domain::repositories::transaction_repository::TransactionRepository;
use cheesy_money_lib::domain::use_cases::list_transactions::list_transactions;
use cheesy_money_lib::infrastructure::persistence::in_memory_transaction_repository::InMemoryTransactionRepository;

// ── Stub helpers ────────────────────────────────────────────────────────────

/// Stub that returns a fixed non-empty list.
struct NonEmptyStubRepo {
    transactions: Vec<Transaction>,
}

impl TransactionRepository for NonEmptyStubRepo {
    fn list(&self) -> Result<Vec<Transaction>, String> {
        Ok(self.transactions.clone())
    }
}

/// Stub that always returns an error.
struct FailingStubRepo {
    message: String,
}

impl TransactionRepository for FailingStubRepo {
    fn list(&self) -> Result<Vec<Transaction>, String> {
        Err(self.message.clone())
    }
}

fn make_transaction(id: &str, amount: i64, description: &str) -> Transaction {
    Transaction {
        id: id.to_string(),
        amount,
        description: description.to_string(),
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

/// SC-contract: given a stub repo returning TWO transactions, list_transactions
/// passes them through unchanged (proves it is not hardcoded to return []).
#[test]
fn list_transactions_passes_through_non_empty_repo_result() {
    let t1 = make_transaction("tx-001", 1000, "Grocery shopping");
    let t2 = make_transaction("tx-002", -500, "Coffee refund");
    let repo = NonEmptyStubRepo {
        transactions: vec![t1.clone(), t2.clone()],
    };

    let result = list_transactions(&repo).expect("expected Ok, got Err");

    assert_eq!(result.len(), 2, "expected 2 transactions");
    assert_eq!(result[0], t1);
    assert_eq!(result[1], t2);
}

/// Edge: single-transaction repo — ensures no off-by-one or slice issues.
#[test]
fn list_transactions_passes_through_single_transaction() {
    let t = make_transaction("tx-single", 9999, "Big purchase");
    let repo = NonEmptyStubRepo {
        transactions: vec![t.clone()],
    };

    let result = list_transactions(&repo).expect("expected Ok");

    assert_eq!(result, vec![t]);
}

/// Error path: repo returns Err → list_transactions propagates it unchanged.
#[test]
fn list_transactions_propagates_repo_error_string() {
    let repo = FailingStubRepo {
        message: "disk full".to_string(),
    };

    let err = list_transactions(&repo).expect_err("expected Err, got Ok");

    assert_eq!(err, "disk full");
}

/// Error path: empty error message propagated (boundary on error content).
#[test]
fn list_transactions_propagates_empty_error_string() {
    let repo = FailingStubRepo {
        message: String::new(),
    };

    let err = list_transactions(&repo).expect_err("expected Err, got Ok");

    assert!(err.is_empty());
}

/// InMemoryTransactionRepository direct test — list() returns Ok([]) at init
/// and the lock is not poisoned.
///
/// Note: InMemoryTransactionRepository::list() acquires Mutex, clones the Vec,
/// and drops the guard — guard is NEVER held across an .await. Verify this
/// property holds by calling list() directly (sync, no await).
#[test]
fn in_memory_repo_list_returns_empty_at_init() {
    let repo = InMemoryTransactionRepository::new();

    let result = repo.list().expect("expected Ok, Mutex should not be poisoned");

    assert!(result.is_empty(), "expected empty store at init");
}

/// Transaction entity: PartialEq + Clone are correctly derived (used by use-case tests).
#[test]
fn transaction_entity_equality_and_clone() {
    let original = make_transaction("tx-eq", 42, "equality check");
    let cloned = original.clone();

    assert_eq!(original, cloned);
    // Different field → not equal
    let different = make_transaction("tx-eq", 42, "DIFFERENT");
    assert_ne!(original, different);
}
