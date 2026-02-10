mod commands;
mod config;
mod linear;
mod rofi;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rofi-linear")]
#[command(about = "Rofi plugin for creating Linear issues")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set up Linear API key (opens browser)
    Auth,
    /// Link a team from Linear
    Link,
    /// Unlink a team
    Unlink {
        /// Team alias to unlink
        team: Option<String>,
    },
    /// List linked teams
    List,
    /// Create a new issue
    Run {
        /// Team alias to use
        team: Option<String>,
        /// Quick mode - title only
        #[arg(short, long)]
        quick: bool,
        /// Open issue in browser after creation
        #[arg(short, long)]
        open_issue: bool,
        /// Multi-team mode - always prompt for team selection
        #[arg(short, long)]
        multi_team: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Auth => commands::auth::run().await,
        Commands::Link => commands::link::run().await,
        Commands::Unlink { team } => commands::unlink::run(team).await,
        Commands::List => commands::list::run().await,
        Commands::Run {
            team,
            quick,
            open_issue,
            multi_team,
        } => commands::run::run(team, quick, open_issue, multi_team).await,
    }
}
