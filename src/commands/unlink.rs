use anyhow::{Context, Result};
use std::io::{self, Write};

use crate::config;

pub async fn run(team: Option<String>) -> Result<()> {
    let alias = match team {
        Some(t) => t,
        None => {
            // List teams and prompt for selection
            let teams = config::list_teams()?;
            if teams.is_empty() {
                println!("No teams linked.");
                return Ok(());
            }

            println!("Linked teams:");
            for (i, (alias, team)) in teams.iter().enumerate() {
                println!("  {}. {} ({})", i + 1, alias, team.name);
            }
            println!();

            print!("Select team to unlink (1-{}): ", teams.len());
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

            teams[selection - 1].0.clone()
        }
    };

    if config::remove_team(&alias)? {
        println!("Team '{}' unlinked.", alias);
    } else {
        println!("Team '{}' not found.", alias);
    }

    Ok(())
}
