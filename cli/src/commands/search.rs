use anyhow::Result;
use crate::api::ApiClient;

pub async fn execute(
    query: &str,
    domains: Option<Vec<String>>,
    limit: usize,
    semantic: bool,
    json_output: bool,
    raw_output: bool,
) -> Result<()> {
    let client = ApiClient::new()?;

    let learnings = if semantic {
        client.search_semantic(query, domains, limit).await?
    } else {
        client.search(query, domains, limit).await?
    };

    if learnings.is_empty() {
        if !raw_output && !json_output {
            eprintln!("No learnings found for query: {}", query);
        }
        return Ok(());
    }

    if json_output {
        println!("{}", serde_json::to_string_pretty(&learnings)?);
    } else if raw_output {
        for learning in &learnings {
            println!("{}", learning.content);
            println!();
        }
    } else {
        let search_type = if semantic { "semantic" } else { "text" };
        println!("Found {} learnings ({} search):\n", learnings.len(), search_type);

        for (i, learning) in learnings.iter().enumerate() {
            let score_str = if semantic {
                learning.similarity
                    .map(|s| format!(" ({:.0}% match)", s * 100.0))
                    .unwrap_or_default()
            } else {
                learning.rank
                    .map(|r| format!(" (rank: {:.2})", r))
                    .unwrap_or_default()
            };

            println!("{}. {} [{}]{}",
                i + 1,
                learning.name,
                learning.slug,
                score_str
            );
            println!("   Domains: {}", learning.domains.join(", "));
            println!();

            for line in learning.content.lines() {
                println!("   {}", line);
            }
            println!();
        }
    }

    Ok(())
}
