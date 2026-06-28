/// IPC-safe error type. Placed outside `domain/` so `serde` does not leak
/// into the pure domain layer. `commands/` maps domain `String` errors here.
#[derive(Debug, thiserror::Error, serde::Serialize)]
#[serde(tag = "kind", content = "message")]
pub enum AppError {
    #[error("repository error: {0}")]
    Repository(String),
}
