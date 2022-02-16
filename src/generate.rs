use crate::Prompt;
use anyhow::{Context, Result};
use std::fs::OpenOptions;
use std::io::Write;
use validator::Validate;

pub fn generate(data: &str, file: &str) -> Result<()> {
    let prompt: Prompt = serde_json::from_str(data)
        .context("Failed to deserialize DATA into the expected format")?;

    prompt
        .validate()
        .context("One or more field validations failed.")?;

    println!(
        "One day Anna was walking her {} {} commute to {} and found a {} {} on the ground.",
        prompt.number, prompt.unit_of_measure, prompt.place, prompt.adjective, prompt.noun
    );

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file)
        .context("Failed to access file")?;
    writeln!(file, "{}", serde_json::to_string(&prompt)?)?;

    Ok(())
}
