use anyhow::Result;

use crate::config;

pub async fn run() -> Result<()> {
    let teams = config::list_teams()?;
    let config = config::load_config()?;

    if teams.is_empty() {
        println!("No teams linked.");
        println!("Run 'rofi-linear link' to link a team.");
        return Ok(());
    }

    println!("Linked teams:");
    for (alias, team) in teams {
        let default_marker = if config.default_team.as_deref() == Some(&alias) {
            " (default)"
        } else {
            ""
        };
        println!("  {} - {}{}", alias, team.name, default_marker);
    }

    Ok(())
}
