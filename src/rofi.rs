use anyhow::{Context, Result};
use std::io::Write;
use std::process::{Command, Stdio};

/// Prompt for text input via rofi
pub fn input(prompt: &str, placeholder: &str) -> Result<Option<String>> {
    let theme_str = format!("entry {{ placeholder: \"{}\"; }}", placeholder);

    let output = Command::new("rofi")
        .args(["-dmenu", "-p", prompt, "-theme-str", &theme_str, "-l", "0"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()
        .context("Failed to run rofi")?;

    if !output.status.success() {
        return Ok(None);
    }

    let text = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 from rofi")?
        .trim()
        .to_string();

    if text.is_empty() {
        Ok(None)
    } else {
        Ok(Some(text))
    }
}

/// Prompt for multiline text input via rofi
pub fn input_multiline(prompt: &str, placeholder: &str) -> Result<Option<String>> {
    let theme_str = format!("entry {{ placeholder: \"{}\"; }}", placeholder);

    let output = Command::new("rofi")
        .args(["-dmenu", "-p", prompt, "-theme-str", &theme_str, "-l", "0"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()
        .context("Failed to run rofi")?;

    if !output.status.success() {
        return Ok(None);
    }

    let text = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 from rofi")?
        .trim()
        .to_string();

    if text.is_empty() {
        Ok(None)
    } else {
        Ok(Some(text))
    }
}

/// Select from a list of options via rofi
pub fn select(prompt: &str, options: &[String]) -> Result<Option<usize>> {
    let mut child = Command::new("rofi")
        .args(["-dmenu", "-p", prompt, "-format", "i"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .context("Failed to spawn rofi")?;

    {
        let stdin = child.stdin.as_mut().context("Failed to get stdin")?;
        for option in options {
            writeln!(stdin, "{}", option)?;
        }
    }

    let output = child.wait_with_output().context("Failed to wait for rofi")?;

    if !output.status.success() {
        return Ok(None);
    }

    let index_str = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 from rofi")?
        .trim()
        .to_string();

    if index_str.is_empty() {
        Ok(None)
    } else {
        let index: usize = index_str.parse().context("Invalid index from rofi")?;
        Ok(Some(index))
    }
}

/// Show an error message via rofi
pub fn error(message: &str) -> Result<()> {
    Command::new("rofi")
        .args(["-e", message])
        .status()
        .context("Failed to show error in rofi")?;
    Ok(())
}

