use anyhow::Result;
use crate::api::ApiClient;

pub async fn execute(domains: Vec<String>, json_output: bool) -> Result<()> {
    let client = ApiClient::new()?;
    let learnings = client.get_by_domains(domains.clone()).await?;

    if learnings.is_empty() {
        eprintln!("No learnings found for domains: {}", domains.join(", "));
        return Ok(());
    }

    if json_output {
        println!("{}", serde_json::to_string_pretty(&learnings)?);
    } else {
        // Output in a format ready to be injected into context
        println!("# Relevant Learnings for: {}\n", domains.join(", "));

        for learning in &learnings {
            println!("## {}", learning.name);
            println!("{}", learning.content);
            println!();
        }
    }

    Ok(())
}
