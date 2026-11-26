use fw_common::{AstDecision, FirewallError, MAX_PAYLOAD_BYTES, SqlAstAnalyzer};

#[derive(Debug, Default)]
pub struct DefaultSqlAnalyzer;

impl SqlAstAnalyzer for DefaultSqlAnalyzer {
    fn analyze_sql(&self, sql: &str) -> Result<AstDecision, FirewallError> {
        if sql.len() > MAX_PAYLOAD_BYTES {
            return Err(FirewallError::PayloadTooLarge);
        }
        // TODO (Phase 1, Days 15-22): Parse via pg_query 18.1.0 and classify into
        // ReadOnlyFastPath / HardBlock / MutateRequiresTwin.
        Ok(AstDecision::ReadOnlyFastPath)
    }
}
