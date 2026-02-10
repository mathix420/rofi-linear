use anyhow::{Context, Result};
use std::io::{self, Write};

use crate::config;
use crate::linear::LinearClient;

pub async fn run() -> Result<()> {
    // Check for API key
    let api_key = config::get_api_key()?.context(
        "No API key found. Run 'rofi-linear auth' first.",
    )?;

    let client = LinearClient::new(&api_key);

    // Fetch teams from Linear
    println!("Fetching teams from Linear...");
    let teams = client.get_teams().await?;

    if teams.is_empty() {
        anyhow::bail!("No teams found in your Linear workspace");
    }

    // Display teams
    println!();
    println!("Available teams:");
    for (i, team) in teams.iter().enumerate() {
        println!("  {}. {} ({})", i + 1, team.name, team.key);
    }
    println!();

    // Prompt for selection
    print!("Select team (1-{}): ", teams.len());
    io::stdout().flush()?;

    let mut selection = String::new();
    io::stdin().read_line(&mut selection)?;
    let selection: usize = selection
        .trim()
        .parse()
        .context("Invalid selection")?;

    if selection < 1 || selection > teams.len() {
        anyhow::bail!("Invalid selection");
    }

    let team = &teams[selection - 1];

    // Prompt for alias
    println!();
    print!("Alias for this team (default: {}): ", team.key.to_lowercase());
    io::stdout().flush()?;

    let mut alias = String::new();
    io::stdin().read_line(&mut alias)?;
    let alias = alias.trim();
    let alias = if alias.is_empty() {
        team.key.to_lowercase()
    } else {
        alias.to_string()
    };

    // Save the team
    config::add_team(&alias, &team.id, &team.name)?;

    println!();
    println!("Team '{}' linked as '{}'!", team.name, alias);

    Ok(())
}
