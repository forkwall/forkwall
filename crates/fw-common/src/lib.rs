use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Hard 64 KiB pre-parse ceiling for Layer 2 SQL and shell payloads.
pub const MAX_PAYLOAD_BYTES: usize = 64 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AstDecision {
    ReadOnlyFastPath,
    MutateRequiresTwin { fingerprint: String },
    HardBlock { reason: String },
}

#[derive(Debug, Error)]
pub enum FirewallError {
    #[error("payload exceeds 64 KiB pre-parse cap")]
    PayloadTooLarge,
    #[error("AST parse error: {0}")]
    ParseError(String),
}

/// Pluggable SQL AST trait (pg_query 18.1.0 now; pg_raw_parse swap seam in Phase 3).
pub trait SqlAstAnalyzer: Send + Sync {
    fn analyze_sql(&self, sql: &str) -> Result<AstDecision, FirewallError>;
}
