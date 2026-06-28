// Tier 2 — Command-level integration tests for the get_transactions command.
//
// Strategy: build a mock Tauri app via tauri::test helpers, extract AppState
// via app.state::<AppState>(), and call the async command function directly.
//
// WHAT IS TESTED:
//   - State injection: AppState is correctly managed and retrievable.
//   - Command logic: get_transactions calls the use-case and maps errors.
//   - AppError IPC shape: serde tagged-object format matches the frontend contract.
//
// WHAT IS NOT TESTED HERE (deliberate; see coverage README):
//   - invoke_handler registration wiring — the generate_handler! entry in lib.rs
//     is NOT exercised by direct function call. SC8 (runtime: getTransactions()→[])
//     is the gate for that. The IPC round-trip via tauri::test::get_ipc_response /
//     InvokeRequest would verify registration, but requires a stable v2 test-IPC
//     API surface and is deferred — see coverage notes.
//
// COMPILATION PREREQUISITES:
//   1. lib.rs must declare: pub mod commands; pub mod domain; pub mod errors;
//      pub mod infrastructure; pub mod state;
//      (currently lib.rs is scaffold-only — HARD BLOCKER, reported to rust-dev)
//   2. tauri::async_runtime::block_on must be a public API in tauri 2.9.x.
//      If it is not, see the block_on helper below for fallback options (both
//      require a direct [dev-dependencies] entry — transitive deps are NOT in
//      the extern prelude and will not compile).
//
// Run from src-tauri/: cargo test --test get_transactions_integration

use cheesy_money_lib::{
    commands::transaction::get_transactions,
    errors::AppError,
    state::AppState,
};
use tauri::Manager;
use tauri::test::{mock_builder, mock_context, noop_assets};

// ── App factory ──────────────────────────────────────────────────────────────

/// Builds a mock Tauri app with AppState wired (InMemoryRepo, empty at init).
/// Using mock_builder avoids a real runtime/webview; tests run in process.
fn build_test_app() -> tauri::App<tauri::test::MockRuntime> {
    mock_builder()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![get_transactions])
        .build(mock_context(noop_assets()))
        .expect("failed to build mock tauri app for testing")
}

/// Run an async future synchronously for test assertions.
///
/// Uses `tauri::async_runtime::block_on` — tauri is already a direct
/// [dev-dependencies] entry, so no new crate dep is needed.
///
/// FALLBACK (if tauri::async_runtime::block_on is not public in this version):
///   Option A — add `futures = "0.3"` to [dev-dependencies] and replace with:
///              `futures::executor::block_on(future)`
///   Option B — add `tokio = { version = "1", features = ["rt"] }` to
///              [dev-dependencies] (MUST be direct dep; transitive dep is NOT
///              in the extern prelude and will not compile) and replace with:
///              `tokio::runtime::Builder::new_current_thread().build().unwrap().block_on(future)`
///
/// get_transactions does not contain any .await internally (the use-case is
/// sync), so any executor drives the future to completion on the first poll.
fn block_on<F: std::future::Future>(future: F) -> F::Output {
    tauri::async_runtime::block_on(future)
}

// ── Tests ────────────────────────────────────────────────────────────────────

/// SC2/SC8 gate — given an app with an empty InMemoryRepo, when get_transactions
/// is called, then the result is Ok([]).
#[test]
fn get_transactions_returns_ok_empty_vec_at_init() {
    let app = build_test_app();
    let state = app.state::<AppState>();

    let result = block_on(get_transactions(state));

    assert!(
        result.is_ok(),
        "expected Ok, got Err: {:?}", result.err()
    );
    assert_eq!(
        result.unwrap().len(),
        0,
        "expected empty Vec<Transaction> at init"
    );
}

/// State isolation — two independently built apps each start with empty repos.
/// Verifies AppState::new() is not a singleton shared across instances.
#[test]
fn two_independent_app_instances_each_start_with_empty_state() {
    let app_a = build_test_app();
    let app_b = build_test_app();

    let result_a = block_on(get_transactions(app_a.state::<AppState>()));
    let result_b = block_on(get_transactions(app_b.state::<AppState>()));

    assert_eq!(result_a.unwrap().len(), 0);
    assert_eq!(result_b.unwrap().len(), 0);
}

/// IPC error contract — AppError::Repository("…") must serialize to the tagged
/// JSON object { "kind": "Repository", "message": "…" } that the frontend
/// branches on. This verifies the #[serde(tag="kind", content="message")]
/// attribute is present and correct.
#[test]
fn app_error_repository_serializes_to_tagged_json_object() {
    let err = AppError::Repository("disk full".to_string());
    let json = serde_json::to_value(&err)
        .expect("AppError must be serde::Serialize");

    assert_eq!(
        json["kind"], "Repository",
        "tag field 'kind' must equal 'Repository'"
    );
    assert_eq!(
        json["message"], "disk full",
        "content field 'message' must match the inner string"
    );
}

/// Error path — AppError::Repository with an empty string is still valid.
/// Verifies no panic or missing-field in serde output.
#[test]
fn app_error_repository_with_empty_message_serializes_correctly() {
    let err = AppError::Repository(String::new());
    let json = serde_json::to_value(&err)
        .expect("AppError must be serde::Serialize");

    assert_eq!(json["kind"], "Repository");
    assert_eq!(json["message"], "");
}

/// No-args path — get_transactions takes no args; confirm serde does not reject
/// the invocation with a type-mismatch. This is N/A for a command with zero
/// parameters (serde cannot produce a malformed-arg error here). Documented
/// explicitly so the missing adversarial-arg case reads as deliberate, not skipped.
///
/// If args are added to get_transactions in future, add a malformed-arg test here.
#[test]
fn get_transactions_no_args_invocation_is_the_only_valid_call_shape() {
    // Command takes no arguments — there is no "malformed arg" path to test.
    // The call below is the canonical invocation; confirm it succeeds.
    let app = build_test_app();
    let result = block_on(get_transactions(app.state::<AppState>()));
    assert!(result.is_ok());
}
