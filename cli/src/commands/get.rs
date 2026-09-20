use anyhow::Result;
use crate::api::ApiClient;

pub async fn execute(slug: &str, json_output: bool, raw_output: bool) -> Result<()> {
    let client = ApiClient::new()?;
    let learning = client.get(slug).await?;

    match learning {
        Some(l) => {
            if json_output {
                println!("{}", serde_json::to_string_pretty(&l)?);
            } else if raw_output {
                println!("{}", l.content);
            } else {
                println!("## {}", l.name);
                println!("Slug: {}", l.slug);
                println!("Domains: {}", l.domains.join(", "));
                println!();
                println!("{}", l.content);
            }
        }
        None => {
            if !raw_output && !json_output {
                eprintln!("Learning not found: {}", slug);
            }
            std::process::exit(1);
        }
    }

    Ok(())
}
