use anyhow::{Context, Result};
use std::process::Command;

use crate::config;
use crate::linear::LinearClient;
use crate::rofi;

fn notify(summary: &str, body: &str) -> bool {
    let output = Command::new("notify-send")
        .args([summary, body, "-A", "default=Open"])
        .output()
        .ok();

    match output {
        Some(o) => String::from_utf8_lossy(&o.stdout).trim() == "default",
        None => false,
    }
}

pub async fn run(team: Option<String>, quick: bool, open_issue: bool) -> Result<()> {
    // Check for API key
    let api_key = config::get_api_key()?.context(
        "No API key found. Run 'rofi-linear auth' first.",
    )?;

    // Get the team to use
    let (_team_alias, team_config) = match team {
        Some(ref t) => config::get_team(Some(t))?,
        None => {
            let teams = config::list_teams()?;
            match teams.len() {
                0 => {
                    rofi::error("No teams linked. Run 'rofi-linear link' first.")?;
                    return Ok(());
                }
                1 => Some(teams.into_iter().next().unwrap()),
                _ => {
                    // Multiple teams - prompt for selection
                    let options: Vec<String> = teams
                        .iter()
                        .map(|(alias, t)| format!("{} ({})", alias, t.name))
                        .collect();

                    match rofi::select("Team", &options)? {
                        Some(idx) => Some(teams.into_iter().nth(idx).unwrap()),
                        None => return Ok(()), // User cancelled
                    }
                }
            }
        }
    }
    .context("Team not found")?;

    // Prompt for issue title
    let title = match rofi::input("Title", "Issue title...")? {
        Some(t) if !t.is_empty() => t,
        _ => return Ok(()), // User cancelled or empty
    };

    // Prompt for description (unless quick mode)
    let description = if quick {
        None
    } else {
        rofi::input_multiline("Description", "Optional description...")?
    };

    // Create the issue
    let client = LinearClient::new(&api_key);
    let issue = match client
        .create_issue(&team_config.id, &title, description.as_deref())
        .await
    {
        Ok(issue) => issue,
        Err(e) => {
            Command::new("notify-send")
                .args(["Linear", &format!("Failed to create issue: {}", e)])
                .spawn()
                .ok();
            return Err(e);
        }
    };

    // Open in browser if requested, or if notification clicked
    if open_issue {
        open::that(&issue.url).ok();
        Command::new("notify-send")
            .args(["Linear", &format!("{} - {}", issue.identifier, issue.title)])
            .spawn()
            .ok();
    } else if notify("Linear", &format!("{} - {}", issue.identifier, issue.title)) {
        open::that(&issue.url).ok();
    }

    Ok(())
}
