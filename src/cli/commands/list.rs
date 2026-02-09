use crate::core::{errors::CliResult, Context};

use crate::core::storage::files::load_recipients;
use crate::core::storage::state::load_current_turn;

use clap::{Arg, ArgMatches, Command};

pub fn cli() -> Command<'static> {
    Command::new("list")
        .about("List recipients and their turn information")
        .arg(
            Arg::new("format")
                .long("format")
                .short('f')
                .help("Output format (table, csv, json)")
                .value_name("FORMAT")
                .default_value("table"),
        )
        .arg(
            Arg::new("current")
                .long("current")
                .help("Show only the current turn organizer")
                .action(clap::ArgAction::SetTrue),
        )
}

pub fn exec(context: &mut Context, args: &ArgMatches) -> CliResult {
    let format = args.get_one::<String>("format").unwrap();
    let current_only = args.get_flag("current");

    // Load recipients from storage
    let recipients = load_recipients(context)?;
    let current_turn = load_current_turn(context)?;

    if current_only {
        // Show only current organizer
        if let Some(organizer) = recipients.iter().find(|r| r.turn == current_turn) {
            println!(
                "Current organizer: {} (Turn {})",
                organizer.email, organizer.turn
            );
        } else {
            println!("No organizer found for turn {}", current_turn);
        }
    } else {
        // Show all recipients
        match format.as_str() {
            "table" => print_table(&recipients, current_turn),
            "csv" => print_csv(&recipients),
            "json" => print_json(&recipients)?,
            _ => return Err("Unsupported format. Use: table, csv, json".into()),
        }
    }

    Ok(())
}

fn print_table(recipients: &[crate::core::types::Recipient], current_turn: u32) {
    println!(
        "{:<30} {:<6} {:<8} {:<8}",
        "Email", "Turn", "Type", "Current"
    );
    println!("{}", "-".repeat(60));

    for recipient in recipients {
        let is_current = if recipient.turn == current_turn {
            "  <--"
        } else {
            ""
        };
        let type_str = if recipient.recipient_type == 1 {
            "Organizer"
        } else {
            "Regular"
        };

        println!(
            "{:<30} {:<6} {:<8} {:<8}",
            recipient.email, recipient.turn, type_str, is_current
        );
    }
}

fn print_csv(recipients: &[crate::core::types::Recipient]) {
    println!("email,turn,recipient_type");
    for recipient in recipients {
        println!(
            "{},{},{}",
            recipient.email, recipient.turn, recipient.recipient_type
        );
    }
}

fn print_json(
    recipients: &[crate::core::types::Recipient],
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(recipients)?);
    Ok(())
}
