use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Learning {
    pub slug: String,
    pub name: String,
    pub content: String,
    pub domains: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct OllamaEmbeddingResponse {
    embedding: Vec<f64>,
}

pub struct ApiClient {
    client: Client,
    config: Config,
}

impl ApiClient {
    pub fn new() -> Result<Self> {
        let config = Config::load()?;
        let client = Client::new();
        Ok(ApiClient { client, config })
    }

    /// Get embedding from Ollama
    async fn get_embedding(&self, text: &str) -> Result<Vec<f64>> {
        #[derive(Serialize)]
        struct OllamaRequest {
            model: String,
            prompt: String,
        }

        let response = self
            .client
            .post("http://localhost:11434/api/embeddings")
            .json(&OllamaRequest {
                model: "nomic-embed-text".to_string(),
                prompt: text.to_string(),
            })
            .send()
            .await
            .context("Failed to connect to Ollama. Is it running?")?;

        let result: OllamaEmbeddingResponse = response
            .json()
            .await
            .context("Failed to parse Ollama response")?;

        Ok(result.embedding)
    }

    /// Semantic search using Ollama embeddings
    pub async fn search_semantic(
        &self,
        query: &str,
        domains: Option<Vec<String>>,
        limit: usize,
    ) -> Result<Vec<Learning>> {
        // Get embedding for query
        let embedding = self.get_embedding(query).await?;

        let url = format!("{}/rest/v1/rpc/search_learnings_semantic", self.config.supabase_url);

        #[derive(Serialize)]
        struct RpcParams {
            query_embedding: Vec<f64>,
            match_threshold: f64,
            match_count: usize,
            #[serde(skip_serializing_if = "Option::is_none")]
            filter_domains: Option<Vec<String>>,
        }

        let params = RpcParams {
            query_embedding: embedding,
            match_threshold: 0.5,
            match_count: limit,
            filter_domains: domains,
        };

        let response = self
            .client
            .post(&url)
            .header("apikey", &self.config.supabase_anon_key)
            .header("Authorization", format!("Bearer {}", self.config.supabase_anon_key))
            .header("Content-Type", "application/json")
            .json(&params)
            .send()
            .await
            .context("Failed to connect to Supabase")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Supabase error: {}", error_text);
        }

        let learnings: Vec<Learning> = response
            .json()
            .await
            .context("Failed to parse response")?;

        Ok(learnings)
    }

    /// Full-text search using PostgreSQL
    pub async fn search(
        &self,
        query: &str,
        domains: Option<Vec<String>>,
        limit: usize,
    ) -> Result<Vec<Learning>> {
        let url = format!("{}/rest/v1/rpc/search_learnings_text", self.config.supabase_url);

        #[derive(Serialize)]
        struct RpcParams {
            query_text: String,
            match_count: usize,
            #[serde(skip_serializing_if = "Option::is_none")]
            filter_domains: Option<Vec<String>>,
        }

        let params = RpcParams {
            query_text: query.to_string(),
            match_count: limit,
            filter_domains: domains,
        };

        let response = self
            .client
            .post(&url)
            .header("apikey", &self.config.supabase_anon_key)
            .header("Authorization", format!("Bearer {}", self.config.supabase_anon_key))
            .header("Content-Type", "application/json")
            .json(&params)
            .send()
            .await
            .context("Failed to connect to Supabase")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Supabase error: {}", error_text);
        }

        let learnings: Vec<Learning> = response
            .json()
            .await
            .context("Failed to parse response")?;

        Ok(learnings)
    }

    /// Get a specific learning by slug
    pub async fn get(&self, slug: &str) -> Result<Option<Learning>> {
        let url = format!(
            "{}/rest/v1/learnings?slug=eq.{}&select=slug,name,content,domains",
            self.config.supabase_url,
            slug
        );

        let response = self
            .client
            .get(&url)
            .header("apikey", &self.config.supabase_anon_key)
            .header("Authorization", format!("Bearer {}", self.config.supabase_anon_key))
            .send()
            .await
            .context("Failed to connect to Supabase")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Supabase error: {}", error_text);
        }

        let learnings: Vec<Learning> = response
            .json()
            .await
            .context("Failed to parse response")?;

        Ok(learnings.into_iter().next())
    }

    /// Get learnings by domains
    pub async fn get_by_domains(&self, domains: Vec<String>) -> Result<Vec<Learning>> {
        let url = format!("{}/rest/v1/rpc/get_learnings_by_domains", self.config.supabase_url);

        #[derive(Serialize)]
        struct RpcParams {
            target_domains: Vec<String>,
        }

        let params = RpcParams {
            target_domains: domains,
        };

        let response = self
            .client
            .post(&url)
            .header("apikey", &self.config.supabase_anon_key)
            .header("Authorization", format!("Bearer {}", self.config.supabase_anon_key))
            .header("Content-Type", "application/json")
            .json(&params)
            .send()
            .await
            .context("Failed to connect to Supabase")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Supabase error: {}", error_text);
        }

        let learnings: Vec<Learning> = response
            .json()
            .await
            .context("Failed to parse response")?;

        Ok(learnings)
    }
}
