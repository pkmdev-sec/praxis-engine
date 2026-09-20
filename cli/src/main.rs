use anyhow::Result;
use clap::{Parser, Subcommand};

mod api;
mod config;
mod commands;

use commands::{search, get, inject};

#[derive(Parser)]
#[command(name = "learn")]
#[command(about = "Praxis Engine CLI - Semantic learning retrieval for AI agents")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search learnings by query
    Search {
        /// The search query
        query: String,

        /// Filter by domains (comma-separated)
        #[arg(short, long)]
        domains: Option<String>,

        /// Number of results (default: 5)
        #[arg(short, long, default_value = "5")]
        limit: usize,

        /// Use semantic search via Ollama embeddings
        #[arg(short, long)]
        semantic: bool,

        /// Output as JSON
        #[arg(long)]
        json: bool,

        /// Output raw content only (for piping)
        #[arg(long)]
        raw: bool,
    },

    /// Get a specific learning by slug
    Get {
        /// The learning slug
        slug: String,

        /// Output as JSON
        #[arg(long)]
        json: bool,

        /// Output raw content only
        #[arg(long)]
        raw: bool,
    },

    /// Inject learnings for specified domains
    Inject {
        /// Domains (comma-separated)
        domains: String,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Search { query, domains, limit, semantic, json, raw } => {
            let domain_list = domains.map(|d| {
                d.split(',').map(|s| s.trim().to_string()).collect()
            });
            search::execute(&query, domain_list, limit, semantic, json, raw).await?;
        }
        Commands::Get { slug, json, raw } => {
            get::execute(&slug, json, raw).await?;
        }
        Commands::Inject { domains, json } => {
            let domain_list: Vec<String> = domains
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            inject::execute(domain_list, json).await?;
        }
    }

    Ok(())
}
