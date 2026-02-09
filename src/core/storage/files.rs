use crate::core::{errors::CliError, types::Recipient, Context};
use log::{debug, info};

/// Load credentials from file
pub fn load_credentials(context: &Context) -> Result<String, CliError> {
    let file_path = context.resources_dir.join("credentials.txt");

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
pub fn load_email_template(context: &Context) -> Result<String, CliError> {
    let file_path = context.resources_dir.join("email_template.txt");
    debug!("Reading email template from: {:?}", file_path);
    let content = std::fs::read_to_string(&file_path)?;
    info!("Email template loaded, {} characters", content.len());
    Ok(content)
}

/// Load recipients from CSV file
pub fn load_recipients(context: &Context) -> Result<Vec<Recipient>, CliError> {
    let file_path = context.resources_dir.join("recipients.csv");

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
