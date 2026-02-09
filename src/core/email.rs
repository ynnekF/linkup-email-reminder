use crate::core::config::*;
use crate::core::types::Recipient;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use log::{debug, error, info, warn};

pub fn send_bulk_email(
    recipients: &[Recipient],
    organizer: &str,
    mailer: &SmtpTransport,
    email_template: &str,
) {
    let mailing_list: Vec<&str> = recipients
        .iter()
        .filter(|r| r.recipient_type == RECIPIENT_TYPE_REGULAR)
        .map(|r| r.email.as_str())
        .collect();

    if mailing_list.is_empty() {
        warn!("No recipients to send to.");
        return;
    }

    info!(
        "Sending bulk email to {} recipients via CC...",
        mailing_list.len()
    );
    debug!("Email CC list: {:?}", mailing_list);

    let mut email_builder = Message::builder()
        .from(organizer.parse().unwrap())
        .subject(EMAIL_SUBJECT);

    for recipient_email in &mailing_list {
        email_builder = email_builder.cc(recipient_email.parse().unwrap());
    }

    let email = email_builder
        .to(organizer.parse().unwrap())
        .body(String::from(email_template))
        .unwrap();

    match mailer.send(&email) {
        Ok(_) => info!(
            "Bulk email sent to {} recipients via CC",
            mailing_list.len()
        ),
        Err(e) => error!("Failed to send bulk email: {:?}", e),
    }
}

/// Create SMTP mailer with credentials
pub fn create_mailer(
    organizer: &str,
    credential: &str,
) -> Result<SmtpTransport, crate::core::errors::CliError> {
    debug!("Creating SMTP mailer for: {}", organizer);

    let creds = Credentials::new(String::from(organizer), String::from(credential));
    let mailer = SmtpTransport::relay(SMTP_SERVER)
        .map_err(|e| crate::core::errors::CliError::from(e.to_string()))?
        .credentials(creds)
        .build();
    Ok(mailer)
}
