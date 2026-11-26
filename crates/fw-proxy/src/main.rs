use fw_common::SqlAstAnalyzer;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .json()
        .init();

    let analyzer = fw_ast::DefaultSqlAnalyzer;
    let decision = analyzer.analyze_sql("SELECT 1");
    tracing::info!(?decision, "forkwall proxy skeleton initialized");
}
