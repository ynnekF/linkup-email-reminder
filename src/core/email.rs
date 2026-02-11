use crate::core::config::*;

use crate::core::storage::files::{load_changelog, load_email_template};
use crate::core::types::Recipient;
use chrono::{DateTime, Datelike, Duration, Local, Timelike, Weekday};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use log::{debug, error, info, warn};

/// Calculate the date of the next Wednesday at midnight
pub fn get_next_wednesday() -> DateTime<Local> {
    let now = Local::now();
    let current_weekday = now.weekday();

    let days_until_wednesday = match current_weekday {
        Weekday::Wed => 7, // If it's Wednesday, get next Wednesday
        Weekday::Thu => 6,
        Weekday::Fri => 5,
        Weekday::Sat => 4,
        Weekday::Sun => 3,
        Weekday::Mon => 2,
        Weekday::Tue => 1,
    };

    // Get the date at midnight (00:00:00) of the next Wednesday
    let next_wednesday_date = now.date_naive() + Duration::days(days_until_wednesday);
    next_wednesday_date
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(Local)
        .unwrap()
}

pub fn build_email(
    current_turn_person: &Recipient,
) -> Result<String, crate::core::errors::CliError> {
    // Read the email template and update it with next current_turn_person
    let mut email_template = load_email_template()?;
    email_template = format!(
        "{}\n\n**This Week's Organizer**: {} (Turn {})\n",
        email_template, current_turn_person.email, current_turn_person.turn,
    );
    // Calculate the next closest Wednesday date and include it in the email template
    let next_wednesday = get_next_wednesday();
    let now = Local::now();
    let hours_until = (next_wednesday - now).num_hours();
    email_template = format!(
        "{}\n**Next Linkup Date**: {} ({} hours until {})\n",
        email_template,
        next_wednesday.format("%Y-%m-%d"),
        hours_until,
        next_wednesday.format("%A")
    );
    email_template = format!("{}\n{}\n", email_template, EMAIL_CLOSER);
    email_template = format!("{}\n{}\n", email_template, SOURCE_CODE);
    email_template = format!("{}\n{}\n", email_template, load_changelog()?);
    Ok(email_template)
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_wednesday_is_at_midnight() {
        let next_wednesday = get_next_wednesday();
        
        // Verify that the time is set to midnight (00:00:00)
        assert_eq!(next_wednesday.hour(), 0);
        assert_eq!(next_wednesday.minute(), 0);
        assert_eq!(next_wednesday.second(), 0);
        
        // Verify that it's a Wednesday
        assert_eq!(next_wednesday.weekday(), Weekday::Wed);
    }

    #[test]
    fn test_next_wednesday_is_in_future() {
        let now = Local::now();
        let next_wednesday = get_next_wednesday();
        
        // Next Wednesday should always be in the future
        assert!(next_wednesday > now);
    }

    #[test]
    fn test_next_wednesday_hours_calculation() {
        let now = Local::now();
        let next_wednesday = get_next_wednesday();
        let hours_until = (next_wednesday - now).num_hours();
        
        // Hours should be between 0 and 168 (7 days * 24 hours)
        // and should never be negative
        assert!(hours_until >= 0);
        assert!(hours_until <= 168);
        
        // If it's currently Tuesday evening (e.g., 7 PM), hours_until should be less than 24
        if now.weekday() == Weekday::Tue && now.hour() >= 19 {
            assert!(hours_until < 24, "On Tuesday evening, next Wednesday should be less than 24 hours away");
        }
    }
}
