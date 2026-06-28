/// Core `Transaction` entity. Derives `Serialize` (outbound IPC only) but not
/// `Deserialize` — the entity only flows outward in `get_transactions`.
/// `amount` is stored as minor units (cents) to avoid floating-point rounding.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Transaction {
    pub id: String,
    pub amount: i64,
    pub description: String,
}
