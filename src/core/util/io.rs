use crate::core::config::*;
use crate::core::types::Recipient;
use std::io::{self, Write};

pub fn get_user_confirmation() -> bool {
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
            Err(_error) => {
                return false;
            }
        }
    }
}

pub fn pretty_print_email_details(
    recipients: &[Recipient],
    organizer: &str,
    _current_turn_person: &Recipient,
    email_template: &str,
) {
    println!("\n{}", "=".repeat(60));
    println!("From: {}", organizer);
    println!("Subject: {}", EMAIL_SUBJECT);

    let cc_list: Vec<&str> = recipients.iter().map(|r| r.email.as_str()).collect();

    println!("To (CC): {}", cc_list.join(", "));
    println!("\nBody:");
    println!("{}", "-".repeat(40));
    println!("{}", email_template);
    println!("{}", "-".repeat(40));
    println!("\n{}\n", "=".repeat(60));
}
