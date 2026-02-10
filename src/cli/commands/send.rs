use crate::core::config::*;
use crate::core::email::{build_email, create_mailer, send_bulk_email};
use crate::core::storage::files::{load_credentials, load_recipients};
use crate::core::storage::state::{get_next_turn, load_current_turn, save_current_turn};
use crate::core::util::io::{get_user_confirmation, pretty_print_email_details};
use crate::core::{errors::CliResult, Context};
use log::info;

use clap::{Arg, ArgMatches, Command};

pub fn cli() -> Command<'static> {
    Command::new("send")
        .about("Notify recipients via email")
        .arg(
            Arg::new("include-ideas")
                .long("include-ideas")
                .short('i')
                .help("Include ideas in the email notification")
                .action(clap::ArgAction::SetFalse),
        )
        .arg(
            Arg::new("debug")
                .long("debug")
                .help("Show email template and quit")
                .action(clap::ArgAction::SetTrue),
        )
}

pub fn exec(context: &mut Context, args: &ArgMatches) -> CliResult {
    let _include_ideas = args.get_flag("include-ideas");
    let debug = args.get_flag("debug");
    let recipients = load_recipients()?;

    // Find organizer (who's sending the email)
    let organizer = recipients
        .iter()
        .find(|r| r.recipient_type == RECIPIENT_TYPE_ORGANIZER)
        .map(|r| r.email.as_str())
        .expect("No organizer found (recipient_type == 1)");
    info!("Organizer identified: {}", organizer);

    // Determine position of current turn
    let current_turn = load_current_turn(context)?;
    let current_turn_person = recipients
        .iter()
        .find(|r| r.turn == current_turn)
        .expect(&format!("No recipient found for turn {}", current_turn));

    info!(
        "Current turn belongs to: {} ({})",
        current_turn_person.email, current_turn_person.turn
    );

    let email_template = build_email(current_turn_person)?;

    // Read gmail app password
    let credential = load_credentials()?;
    let mailer = create_mailer(organizer, &credential)?;

    pretty_print_email_details(&recipients, organizer, current_turn_person, &email_template);

    if debug {
        info!("Debug mode - email template shown above, quitting without sending");
        let debug_file_path = context.resources_dir.join("debug_email_template.md");
        std::fs::write(&debug_file_path, &email_template)?;
        info!("Email template also written to: {:?}", debug_file_path);
        return Ok(());
    }
    // Confirm + send
    if get_user_confirmation() {
        send_bulk_email(&recipients, organizer, &mailer, &email_template);

        let next_turn = get_next_turn(&recipients, current_turn);
        save_current_turn(context, next_turn)?;
    } else {
        info!("User cancelled - email not sent");
    }
    Ok(())
}
