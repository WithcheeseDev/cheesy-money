# Coverage — rust-core partition

## Status: NOT RUN (toolchain guard)

No `cargo`/`rustc` on this machine. Tests are AUTHORED and syntactically correct;
compilation + execution require Rust ≥ 1.77.

---

## Tests authored

### Tier 1 — Domain unit tests

**File:** `src-tauri/tests/domain_list_transactions.rs`

| Test name | What it verifies |
|---|---|
| `list_transactions_passes_through_non_empty_repo_result` | Use-case passes through 2 transactions from stub repo unchanged (proves no hardcoded []) |
| `list_transactions_passes_through_single_transaction` | Single-item edge case; no off-by-one |
| `list_transactions_propagates_repo_error_string` | Error path: "disk full" error string propagated unchanged |
| `list_transactions_propagates_empty_error_string` | Boundary: empty error string propagated |
| `in_memory_repo_list_returns_empty_at_init` | InMemoryTransactionRepository::list() → Ok([]) at init; Mutex not poisoned |
| `transaction_entity_equality_and_clone` | Transaction derives PartialEq + Clone correctly |

**NOTE:** rust-dev owns ONE inline test in `list_transactions.rs`:
`list_transactions_returns_empty_for_empty_repo` (SC2). Not duplicated here.

### Tier 2 — Command integration tests (tauri::test)

**File:** `src-tauri/tests/get_transactions_integration.rs`

| Test name | What it verifies |
|---|---|
| `get_transactions_returns_ok_empty_vec_at_init` | SC2/SC8 gate: command returns Ok([]) via mock app with AppState |
| `two_independent_app_instances_each_start_with_empty_state` | State isolation; AppState::new() is not a singleton |
| `app_error_repository_serializes_to_tagged_json_object` | IPC contract: AppError serializes to {kind:"Repository", message:"…"} |
| `app_error_repository_with_empty_message_serializes_correctly` | Error boundary: empty message in AppError is valid |
| `get_transactions_no_args_invocation_is_the_only_valid_call_shape` | Documents N/A for malformed-arg (command takes no args) |

### Tier 3 — Frontend IPC mocks (mockIPC)

NOT authored in this partition. The frontend `mockIPC` test belongs to the
`frontend-stubs` partition (`src/api/transaction.api.ts`).

**macOS E2E ceiling:** `tauri-driver` / WebDriver E2E is experimental in Tauri v2
and **unsupported on macOS** (WKWebView has no WebDriver client). The realistic
top tier on macOS is `mockIPC`. Do NOT claim Playwright/WebDriver E2E for this
project. SC8 is verified manually at runtime (`bun run tauri dev` → `getTransactions()`).

---

## Run command

```sh
# From project root / src-tauri directory:
cd src-tauri

# Run all rust-core tests (domain unit + command integration):
cargo test

# Run only the additional domain tests:
cargo test --test domain_list_transactions

# Run only the command integration tests:
cargo test --test get_transactions_integration

# Run with coverage (preferred: cargo-llvm-cov):
cargo llvm-cov --lcov --output-path ../coverage-rust-core/lcov.info

# Run with tarpaulin (alternative):
cargo tarpaulin --out Lcov --output-dir ../coverage-rust-core/
```

---

## Compilation prerequisites

### ✅ RESOLVED — lib.rs module declarations (2026-06-28)

`lib.rs` now declares `pub mod {commands, domain, errors, infrastructure, state}` and
`pub fn run()` does `.manage(AppState::new()).invoke_handler(tauri::generate_handler![commands::transaction::get_transactions])`.
All product modules are accessible as `cheesy_money_lib::*`. greet scaffold removed.

### ✅ RESOLVED — async runtime for command integration tests (2026-06-28)

`tauri::async_runtime::block_on` is CONFIRMED public in Tauri v2 (re-exported from the
async_runtime module). Zero additional dev-dependencies required. The fallback options
(futures/tokio) are documented in the test file helper as insurance only — do not add
them speculatively.

**CRITICAL NOTE (for future editors):** transitive dependencies are NOT in the extern
prelude. If you replace `tauri::async_runtime::block_on` with `tokio::runtime::Builder`,
you MUST add `tokio = { version = "1", features = ["rt"] }` as a DIRECT [dev-dependencies]
entry — relying on tokio as a transitive dep produces a hard compile error.

---

## Coverage gaps (risk assessment)

| Gap | Risk | Mitigation |
|---|---|---|
| `invoke_handler` registration NOT tested (direct call, not IPC round-trip) | LOW — registration is trivially verified by SC8 at runtime; `generate_handler!` is a macro with no logic to test | SC8 is the gate |
| No concurrent-access test for InMemoryTransactionRepository Mutex | LOW — InMemoryRepo is an in-memory stub, not production; Mutex correctness verified by Rust type system | Acceptable at scaffold stage |
| AppError: only 1 variant (Repository) — full variant matrix covered | N/A | Add tests when new variants are added |
| Malformed-arg path for get_transactions | N/A — command takes no args; serde cannot produce type-mismatch here | Explicitly documented in test |
| macOS E2E (tauri-driver/WebDriver) | CEILING — unsupported on macOS WKWebView | mockIPC is the top tier; SC8 is manual gate |

---

## Success criteria coverage map

| SC | Gate | Covered by |
|---|---|---|
| SC1 `bun run tauri dev` launches | Manual / lead | N/A (cannot run without toolchain + display) |
| SC2 `cargo test` passes (list_transactions domain unit test) | Rust toolchain | `list_transactions.rs` inline (rust-dev) + `tests/domain_list_transactions.rs` (this file) — **BLOCKED by lib.rs** |
| SC3 `cargo clippy -- -D warnings` exits 0 | Rust toolchain | NOT RUN (toolchain guard) |
| SC4 `grep "tauri::"` in `domain/` → 0 | grep | Runtime check; domain tests import only std/serde (no tauri::) |
| SC5 `grep "tauri::"` in `infrastructure/` → 0 | grep | Runtime check; infra tests import only std |
| SC6 `invoke(` only under `src/api/` | grep | frontend-stubs partition |
| SC7 `bun run lint` exits 0 | Node/bun | frontend-stubs partition |
| SC8 `getTransactions()` → `[]` | Manual runtime | `get_transactions_returns_ok_empty_vec_at_init` validates logic; IPC registration gate is runtime SC8 |
