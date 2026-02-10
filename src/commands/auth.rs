use anyhow::{Context, Result};
use std::io::{self, Write};

use crate::config;
use crate::linear::LinearClient;

pub async fn run() -> Result<()> {
    // Open browser to Linear API key settings
    println!("Opening Linear API key settings in browser...");
    open::that("https://linear.app/settings/account/security")
        .context("Failed to open browser")?;

    println!();
    println!("Create a new API key and paste it below.");
    println!("(The key should start with 'lin_api_')");
    println!();

    // Prompt for API key
    print!("API Key: ");
    io::stdout().flush()?;

    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key)?;
    let api_key = api_key.trim();

    if api_key.is_empty() {
        anyhow::bail!("No API key provided");
    }

    // Validate the key by fetching the viewer
    println!();
    println!("Validating API key...");

    let client = LinearClient::new(api_key);
    let viewer = client
        .get_viewer()
        .await
        .context("Failed to validate API key - please check it's correct")?;

    // Save the key
    config::set_api_key(api_key)?;

    println!();
    println!("Success! Authenticated as {} ({})", viewer.name, viewer.email);
    println!();
    println!("Next steps:");
    println!("  1. Link a team: rofi-linear link");
    println!("  2. Create an issue: rofi-linear run");

    Ok(())
}
