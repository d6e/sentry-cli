use crate::config::{config_path, load_config};
use crate::error::{Result, SentryCliError};
use crate::output::print_success;
use std::fs;
use std::io::{self, Write};

pub fn init_config() -> Result<()> {
    let path = config_path();

    if path.exists() {
        print!(
            "Config file already exists at {}. Overwrite? [y/N] ",
            path.display()
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            return Ok(());
        }
    }

    println!("Sentry CLI Configuration");
    println!("========================\n");

    // Get auth token
    print!("Enter your Sentry auth token (create one at https://sentry.io/settings/account/api/auth-tokens/): ");
    io::stdout().flush()?;

    let mut auth_token = String::new();
    io::stdin().read_line(&mut auth_token)?;
    let auth_token = auth_token.trim();

    if auth_token.is_empty() {
        return Err(SentryCliError::Auth("Auth token is required".to_string()));
    }

    // Get default organization (optional)
    print!("Enter default organization slug [optional]: ");
    io::stdout().flush()?;

    let mut default_org = String::new();
    io::stdin().read_line(&mut default_org)?;
    let default_org = default_org.trim();

    // Get default project (optional)
    print!("Enter default project slug [optional]: ");
    io::stdout().flush()?;

    let mut default_project = String::new();
    io::stdin().read_line(&mut default_project)?;
    let default_project = default_project.trim();

    // Get server URL (optional, for self-hosted)
    print!("Enter Sentry server URL [leave empty for sentry.io]: ");
    io::stdout().flush()?;

    let mut server_url = String::new();
    io::stdin().read_line(&mut server_url)?;
    let server_url = server_url.trim();

    // Create parent directory if needed
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Build config content
    let mut config_content = format!("auth_token = \"{}\"\n", auth_token);
    if !default_org.is_empty() {
        config_content.push_str(&format!("default_org = \"{}\"\n", default_org));
    }
    if !default_project.is_empty() {
        config_content.push_str(&format!("default_project = \"{}\"\n", default_project));
    }
    if !server_url.is_empty() {
        config_content.push_str(&format!("server_url = \"{}\"\n", server_url));
    }

    fs::write(&path, config_content)?;

    println!("\nConfig saved to {}", path.display());
    println!("You can now use 'sentry' commands!");

    Ok(())
}

pub fn show_config() -> Result<()> {
    let config = load_config();
    let path = config_path();

    println!("Config file: {}", path.display());
    println!();

    if let Some(org) = &config.default_org {
        println!("default_org:     {}", org);
    }

    if let Some(server) = &config.server_url {
        println!("server_url:      {}", server);
    } else {
        println!("server_url:      https://sentry.io (default)");
    }

    if let Some(_token) = &config.auth_token {
        println!("auth_token:      ****... (set in config)");
    } else if std::env::var("SENTRY_AUTH_TOKEN").is_ok() {
        println!("auth_token:      ****... (from SENTRY_AUTH_TOKEN)");
    } else {
        println!("auth_token:      (not set)");
    }

    if let Some(project) = &config.default_project {
        println!("default_project: {}", project);
    }

    Ok(())
}

pub fn set_config(key: &str, value: &str) -> Result<()> {
    let path = config_path();
    let mut config = load_config();

    match key {
        "default_org" => config.default_org = Some(value.to_string()),
        "server_url" => config.server_url = Some(value.to_string()),
        "auth_token" => config.auth_token = Some(value.to_string()),
        "default_project" => config.default_project = Some(value.to_string()),
        _ => {
            return Err(SentryCliError::Validation(format!(
                "Unknown config key: {}. Valid keys: default_org, server_url, auth_token, default_project",
                key
            )))
        }
    }

    // Create parent directory if needed
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let toml_content = toml::to_string_pretty(&config)
        .map_err(|e| SentryCliError::Config(format!("Failed to serialize config: {}", e)))?;

    fs::write(&path, toml_content)?;

    // Don't print sensitive values like auth tokens
    if key == "auth_token" {
        print_success(&format!("Updated {}", key));
    } else {
        print_success(&format!("Updated {} to \"{}\"", key, value));
    }

    Ok(())
}
