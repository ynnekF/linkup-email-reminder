use crate::core::{config::*, errors::CliError, types::Recipient};
use log::{debug, info};
use std::path::PathBuf;

/// Load credentials from file
pub fn load_credentials() -> Result<String, CliError> {
    let file_path = PathBuf::from(PRIVATE_DIR).join("credentials.txt");

    let content = std::fs::read_to_string(&file_path)?;
    let credentials: Vec<String> = content
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();
    info!("Successfully loaded {} credentials", credentials.len());

    let credential = credentials.first().expect("No credentials found in file");
    Ok(credential.clone())
}

/// Load email template from file
pub fn load_email_template() -> Result<String, CliError> {
    let file_path = PathBuf::from(RESOURCES_DIR).join("email_template.md");
    debug!("Reading email template from: {:?}", file_path);
    let content = std::fs::read_to_string(&file_path)?;
    info!("Email template loaded, {} characters", content.len());
    Ok(content)
}

/// Load recipients from CSV file
pub fn load_recipients() -> Result<Vec<Recipient>, CliError> {
    let file_path = PathBuf::from(PRIVATE_DIR).join("recipients.csv");

    if !file_path.exists() {
        return Err(format!("Recipients file not found: {}", file_path.display()).into());
    }

    let mut csv_reader = csv::Reader::from_path(&file_path)?;
    let mut recipients = Vec::new();

    for result in csv_reader.deserialize() {
        let recipient: Recipient = result?;
        recipients.push(recipient);
    }

    for recipient in &recipients {
        debug!(
            "Recipient - Email: {}, Turn: {}, Type: {}",
            recipient.email, recipient.turn, recipient.recipient_type
        );
    }

    Ok(recipients)
}

pub fn load_changelog() -> Result<String, CliError> {
    let file_path = PathBuf::from(DOCS_DIR).join("CHANGELOG.md");
    debug!("Reading changelog from: {:?}", file_path);
    let content = std::fs::read_to_string(&file_path)?;
    info!("Changelog loaded, {} characters", content.len());

    // Read only first 5 change entries (split by "## [")
    let entries: Vec<&str> = content.split("## [").collect();
    println!("Total changelog entries found: {}", entries.len());
    let recent_entries = entries
        .into_iter()
        .take(5)
        .collect::<Vec<&str>>()
        .into_iter()
        .collect::<Vec<&str>>()
        .join("## [");
    Ok(recent_entries)
}
