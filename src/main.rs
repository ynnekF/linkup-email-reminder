mod config;
mod email;
mod file_utils;
mod types;

use config::*;
use email::*;
use file_utils::*;
use log::{debug, error, info, warn};
use std::io::{self, Write};
use std::path::Path;
use types::Recipient;

fn get_user_confirmation() -> bool {
    loop {
        print!("\nDo you want to send this email? (y/n): ");
        io::stdout().flush().unwrap(); // display immediately
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim().to_lowercase();
                match input.as_str() {
                    "y" | "yes" => {
                        return true;
                    }
                    "n" | "no" => {
                        return false;
                    }
                    _ => {
                        println!("Please enter 'y' for yes or 'n' for no.");
                        continue;
                    }
                }
            }
            Err(error) => {
                return false;
            }
        }
    }
}

fn get_next_turn(recipients: &[Recipient], current_turn: u32) -> u32 {
    let max_turn = recipients.iter().map(|r| r.turn).max().unwrap_or(1);
    if current_turn >= max_turn {
        1
    } else {
        current_turn + 1
    }
}

fn main() {
    env_logger::init();
    
    const VERSION: &str = "1.0.0";
    info!("Starting email-reminder application - v{}", VERSION);

    let credential_file = Path::new(CREDENTIALS_FILE);
    let recipients_file = Path::new(RECIPIENTS_FILE);
    let b_template_file = Path::new(EMAIL_TEMPLATE_FILE);

    let credentials = read_credentials_from_file(credential_file)
        .expect("Failed to read credentials");
    let credential = credentials.first().expect("No credentials found in file");

    let recipients: Vec<Recipient> = read_recipients_from_csv(recipients_file)
        .expect("Failed to read recipients from CSV");

    for recipient in &recipients {
        debug!(
            "Recipient - Email: {}, Turn: {}, Type: {}",
            recipient.email, recipient.turn, recipient.recipient_type
        );
    }

    let organizer = recipients
        .iter()
        .find(|r| r.recipient_type == RECIPIENT_TYPE_ORGANIZER)
        .map(|r| r.email.as_str())
        .expect("No organizer found (recipient_type == 1)");

    info!("Organizer identified: {}", organizer);

    let current_turn =
        read_current_turn(Path::new(TURN_STATE_FILE)).expect("Failed to read current turn");

    let current_turn_person = recipients
        .iter()
        .find(|r| r.turn == current_turn)
        .expect(&format!("No recipient found for turn {}", current_turn));

    info!(
        "Current turn belongs to: {} ({})",
        current_turn_person.email, current_turn_person.turn
    );

    let email_template =
        read_email_template(b_template_file).expect("Failed to read email template");

    // Add turn information to email template
    let updated_email_template = format!(
        "{}\n\n**This Week's Organizer**: {} (Turn {})\n\n",
        email_template,
        current_turn_person.email,
        current_turn_person.turn,
    );

    let mailer = create_mailer(organizer, credential).expect("Failed to create SMTP mailer");

    pretty_print_email_details(
        &recipients,
        organizer,
        current_turn_person,
        &updated_email_template,
    );

    if get_user_confirmation() {
        send_bulk_email(&recipients, organizer, &mailer, &updated_email_template);

        let next_turn = get_next_turn(&recipients, current_turn);
        save_current_turn(Path::new(TURN_STATE_FILE), next_turn).expect("Failed to save next turn");
    } else {
        info!("User cancelled - email not sent");
    }
}
