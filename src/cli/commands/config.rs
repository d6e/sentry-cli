use crate::config::{config_path, load_config};
use crate::error::{Result, SentryCliError};
use crate::output::print_success;
use std::fs;

pub fn init_config() -> Result<()> {
    let path = config_path();

    if path.exists() {
        return Err(SentryCliError::Config(format!(
            "Config file already exists at {}",
            path.display()
        )));
    }

    // Create parent directory if needed
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let default_config = r#"# Sentry CLI Configuration

# Default organization slug
# default_org = "my-organization"

# Sentry server URL (for self-hosted instances)
# server_url = "https://sentry.io"

# Auth token (SENTRY_AUTH_TOKEN env var takes precedence)
# auth_token = "sntrys_..."

# Default project slug
# default_project = "my-project"
"#;

    fs::write(&path, default_config)?;
    print_success(&format!("Created config file at {}", path.display()));
    println!("Edit the file to add your auth token and organization.");

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
    print_success(&format!("Updated {} to \"{}\"", key, value));

    Ok(())
}
