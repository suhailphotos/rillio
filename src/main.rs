use reqwest::blocking::Client;
use std::{env, fs, path::PathBuf};

const NOTION_PAGE_ID: &str = "2a2a1865b18781fca571e00c54ef8006";
const NOTION_VERSION: &str = "2025-09-03";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (config_path, secrets_path, created_config, created_secrets) = bootstrap_config_files()?;

    if created_config {
        eprintln!("Created {}", config_path.display());
    }
    if created_secrets {
        eprintln!("Created {}", secrets_path.display());
    }

    let api_key = match env::var("NOTION_API_KEY") {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Error: NOTION_API_KEY environment variable is not set");
            eprintln!("Hint: export NOTION_API_KEY='your-secret-notion-key'");
            eprintln!("(You now also have {} where you can choose to manage secrets later.)", secrets_path.display());
            std::process::exit(1);
        }
    };

    let url = format!("https://api.notion.com/v1/pages/{NOTION_PAGE_ID}");
    let client = Client::new();

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Notion-Version", NOTION_VERSION)
        .send()?; // blocking

    println!("Status {}", response.status());

    let body = response.text()?; // raw JSON as String
    println!("{body}");

    Ok(())
}

fn bootstrap_config_files() -> Result<(PathBuf, PathBuf, bool, bool), Box<dyn std::error::Error>> {
    let base = config_root()?;
    let rillio_dir = base.join("rillio");
    fs::create_dir_all(&rillio_dir)?;

    let config_path = rillio_dir.join("config.toml");
    let secrets_path = rillio_dir.join("secrets.env");

    let created_config = if !config_path.exists() {
        fs::File::create(&config_path)?; // blank
        true
    } else {
        false
    };

    let created_secrets = if !secrets_path.exists() {
        fs::File::create(&secrets_path)?; // blank
        // Tighten permissions on Unix: rw------- (600)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&secrets_path, fs::Permissions::from_mode(0o600))?;
        }
        true
    } else {
        false
    };

    Ok((config_path, secrets_path, created_config, created_secrets))
}

fn config_root() -> Result<PathBuf, Box<dyn std::error::Error>> {
    if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
        if !xdg.is_empty() {
            return Ok(PathBuf::from(xdg));
        }
    }

    #[cfg(target_os = "windows")]
    {
        if let Ok(appdata) = env::var("APPDATA") {
            if !appdata.is_empty() {
                return Ok(PathBuf::from(appdata));
            }
        }
    }

    if let Ok(home) = env::var("HOME") {
        return Ok(PathBuf::from(home).join(".config"));
    }

    Err("Could not determine config directory (XDG_CONFIG_HOME/APPDATA/HOME missing)".into())
}
