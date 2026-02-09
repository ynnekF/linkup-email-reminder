use crate::core::{errors::CliError, types::Recipient, Context};
use log::{debug, info};

/// Load current turn number from file
pub fn load_current_turn(context: &Context) -> Result<u32, CliError> {
    let file_path = context.resources_dir.join("current_turn.txt");

    if !file_path.exists() {
        return Ok(1); // Default to turn 1 if file doesn't exist
    }

    let content = std::fs::read_to_string(&file_path)?;
    let turn = content
        .trim()
        .parse::<u32>()
        .map_err(|_| CliError::from("Invalid turn number in current_turn.txt"))?;

    Ok(turn)
}

/// Save current turn number to file
pub fn save_current_turn(context: &Context, turn: u32) -> Result<(), CliError> {
    let file_path = context.resources_dir.join("current_turn.txt");
    debug!("Saving turn {} to: {:?}", turn, file_path);

    // Create resources directory if it doesn't exist
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&file_path, turn.to_string())?;
    info!("Turn {} saved successfully", turn);
    Ok(())
}

/// Calculate the next turn number
pub fn get_next_turn(recipients: &[Recipient], current_turn: u32) -> u32 {
    let max_turn = recipients.iter().map(|r| r.turn).max().unwrap_or(1);
    if current_turn >= max_turn {
        1
    } else {
        current_turn + 1
    }
}
