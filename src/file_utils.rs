use crate::types::Recipient;
use log::{debug, info};
use std::error::Error;
use std::fs::File;
use std::path::Path;

pub fn read_recipients_from_csv(file_path: &Path) -> Result<Vec<Recipient>, Box<dyn Error>> {
    debug!("Opening CSV file: {:?}", file_path);
    let file = File::open(file_path)?;
    let mut csv_reader = csv::Reader::from_reader(file);
    let mut recipients = Vec::new();
    for result in csv_reader.deserialize() {
        let recipient: Recipient = result?;
        recipients.push(recipient);
    }
    info!("Successfully read {} recipients from CSV", recipients.len());
    Ok(recipients)
}

pub fn read_credentials_from_file(file_path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    debug!("Reading credentials from: {:?}", file_path);
    let content = std::fs::read_to_string(file_path)?;
    let credentials: Vec<String> = content
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();
    info!("Successfully loaded {} credentials", credentials.len());
    Ok(credentials)
}

pub fn read_email_template(file_path: &Path) -> Result<String, Box<dyn Error>> {
    debug!("Reading email template from: {:?}", file_path);
    let content = std::fs::read_to_string(file_path)?;
    info!("Email template loaded, {} characters", content.len());
    Ok(content)
}

pub fn read_current_turn(file_path: &Path) -> Result<u32, Box<dyn Error>> {
    debug!("Reading current turn from: {:?}", file_path);

    if !file_path.exists() {
        info!("Turn state file doesn't exist, starting with turn 1");
        return Ok(1);
    }

    let content = std::fs::read_to_string(file_path)?;
    let turn = content.trim().parse::<u32>()?;
    info!("Current turn loaded: {}", turn);
    Ok(turn)
}

pub fn save_current_turn(file_path: &Path, turn: u32) -> Result<(), Box<dyn Error>> {
    debug!("Saving turn {} to: {:?}", turn, file_path);

    // Create resources directory if it doesn't exist
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(file_path, turn.to_string())?;
    info!("Turn {} saved successfully", turn);
    Ok(())
}
