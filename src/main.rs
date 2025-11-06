use reqwest::blocking::Client;
use std::env;

const NOTION_PAGE_ID: &str = "2a2a1865b18781fca571e00c54ef8006";
const NOTION_VERSION: &str = "2025-09-03";

fn main() -> Result<(), Box<dyn std::error::Error>> {

  let api_key = match env::var("NOTION_API_KEY" ) {
    Ok(value) => value,
    Err(_) => {
      eprintln!("ERROR: NOTION_API_KEY environment variable is not set.");
      eprintln!("Hint: export NOTION_API_KEY='your-secret-key-here'");
      std::process::exit(1);
    },
  };

  let url = format!("https://api.notion.com/v1/pages/{NOTION_PAGE_ID}");

  let client = Client::new();

  let response = client
    .get(&url)
    .header("Authorization", format!("Bearer {}", api_key))
    .header("Notion-Version", NOTION_VERSION)
    .send()?;

  println!("Status: {}", response.status());

  let body = response.text()?;
  println!("{body}");
  Ok(())
}

