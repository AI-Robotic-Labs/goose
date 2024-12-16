use anyhow::Result;
use mcp_server::{Router, Server, ByteTransport};
use tokio::io::{stdin, stdout};
use tokio::time::Duration;
use tower::timeout::Timeout;
use tracing_subscriber::{self, EnvFilter};
use tracing_appender::rolling::{RollingFileAppender, Rotation};

#[tokio::main]
async fn main() -> Result<()> {
    // Set up file appender
    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        "logs",
        "mcp-server.log",
    );

    // Initialize the tracing subscriber with file and stdout logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env()
            .add_directive(tracing::Level::INFO.into()))
        .with_writer(file_appender)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    tracing::info!("Starting MCP server");

    let router = Router::new();
    // Add a 30 second timeout to all requests
    let router = Timeout::new(router, Duration::from_secs(30));
    let server = Server::new(router);
    
    let transport = ByteTransport::new(stdin(), stdout());
    Ok(server.run(transport).await?)
}