mod api;
mod auth;
mod config;
mod db;
mod error;
mod models;
mod parsers;
mod services;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "finance-tracker", about = "Personal finance tracker")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Import a bank CSV file
    Import {
        /// Path to the CSV file
        file: PathBuf,
    },
    /// Start the HTTP API server
    Serve,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "finance_tracker=info".into()),
        )
        .init();

    let cfg = config::Config::from_env();
    let pool = db::create_pool(&cfg.database_url).await?;
    db::run_migrations(&pool).await?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Import { file } => {
            let filename = file
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_else(|| "unknown".into());

            let content = std::fs::read_to_string(&file)
                .map_err(|e| format!("Failed to read {}: {}", file.display(), e))?;

            tracing::info!("Importing file: {}", file.display());

            let result = services::import::import_file(&pool, &filename, &content).await?;

            println!(
                "Import complete: {} total, {} imported, {} skipped (duplicates)",
                result.total_rows, result.imported, result.skipped
            );
        }
        Commands::Serve => {
            let router = api::create_router(pool, cfg.clone());

            let listener = tokio::net::TcpListener::bind(&cfg.bind_addr).await?;
            tracing::info!("Listening on {}", cfg.bind_addr);
            axum::serve(listener, router).await?;
        }
    }

    Ok(())
}
