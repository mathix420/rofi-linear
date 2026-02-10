use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub default_team: Option<String>,
    #[serde(default)]
    pub teams: HashMap<String, TeamConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TeamConfig {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Creds {
    pub api_key: Option<String>,
}

fn config_dir() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .context("Could not find config directory")?
        .join("rofi-linear");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
        // Create .gitignore for creds
        fs::write(config_dir.join(".gitignore"), "creds.yaml\n")?;
    }

    Ok(config_dir)
}

fn config_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("config.yaml"))
}

fn creds_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("creds.yaml"))
}

pub fn load_config() -> Result<Config> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(Config::default());
    }
    let content = fs::read_to_string(&path)?;
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config)
}

pub fn save_config(config: &Config) -> Result<()> {
    let path = config_path()?;
    let content = serde_yaml::to_string(config)?;
    fs::write(path, content)?;
    Ok(())
}

pub fn load_creds() -> Result<Creds> {
    let path = creds_path()?;
    if !path.exists() {
        return Ok(Creds::default());
    }
    let content = fs::read_to_string(&path)?;
    let creds: Creds = serde_yaml::from_str(&content)?;
    Ok(creds)
}

pub fn save_creds(creds: &Creds) -> Result<()> {
    let path = creds_path()?;
    let content = serde_yaml::to_string(creds)?;
    fs::write(path, content)?;
    Ok(())
}

pub fn get_api_key() -> Result<Option<String>> {
    Ok(load_creds()?.api_key)
}

pub fn set_api_key(key: &str) -> Result<()> {
    let mut creds = load_creds()?;
    creds.api_key = Some(key.to_string());
    save_creds(&creds)
}

pub fn add_team(alias: &str, id: &str, name: &str) -> Result<()> {
    let mut config = load_config()?;
    config.teams.insert(
        alias.to_string(),
        TeamConfig {
            id: id.to_string(),
            name: name.to_string(),
        },
    );
    // Set as default if first team
    if config.default_team.is_none() {
        config.default_team = Some(alias.to_string());
    }
    save_config(&config)
}

pub fn remove_team(alias: &str) -> Result<bool> {
    let mut config = load_config()?;
    let removed = config.teams.remove(alias).is_some();
    if removed {
        // Clear default if it was the removed team
        if config.default_team.as_deref() == Some(alias) {
            config.default_team = config.teams.keys().next().cloned();
        }
        save_config(&config)?;
    }
    Ok(removed)
}

pub fn get_team(alias: Option<&str>) -> Result<Option<(String, TeamConfig)>> {
    let config = load_config()?;

    let alias = alias
        .map(String::from)
        .or(config.default_team)
        .context("No team specified and no default team set")?;

    Ok(config
        .teams
        .get(&alias)
        .map(|t| (alias.clone(), t.clone())))
}

pub fn list_teams() -> Result<Vec<(String, TeamConfig)>> {
    let config = load_config()?;
    Ok(config
        .teams
        .into_iter()
        .collect())
}
