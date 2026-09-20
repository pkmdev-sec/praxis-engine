use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;

pub struct Config {
    pub supabase_url: String,
    pub supabase_anon_key: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        // Try to load from .env files in order of precedence
        let possible_paths = [
            // Current directory
            PathBuf::from(".env"),
            // Home directory praxis-engine
            dirs::home_dir()
                .map(|h| h.join("praxis-engine/.env"))
                .unwrap_or_default(),
            // Config directory
            dirs::config_dir()
                .map(|c| c.join("praxis-engine/.env"))
                .unwrap_or_default(),
        ];

        for path in possible_paths.iter() {
            if path.exists() {
                let _ = dotenvy::from_path(path);
                break;
            }
        }

        let supabase_url = env::var("SUPABASE_URL")
            .context("SUPABASE_URL not set. Set in .env or environment.")?;

        let supabase_anon_key = env::var("SUPABASE_ANON_KEY")
            .context("SUPABASE_ANON_KEY not set. Set in .env or environment.")?;

        Ok(Config {
            supabase_url,
            supabase_anon_key,
        })
    }
}
