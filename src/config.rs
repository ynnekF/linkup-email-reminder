// Resource files
pub const CREDENTIALS_FILE: &str = "resources/credentials.txt";
pub const RECIPIENTS_FILE: &str = "resources/recipients.csv";
pub const EMAIL_TEMPLATE_FILE: &str = "resources/email_template.txt";
pub const TURN_STATE_FILE: &str = "resources/current_turn.txt";

// Email configuration
pub const SMTP_SERVER: &str = "smtp.gmail.com";
pub const EMAIL_SUBJECT: &str = "Weekly Linkup Reminder";

// Recipient types
pub const RECIPIENT_TYPE_REGULAR: u8 = 0;
pub const RECIPIENT_TYPE_ORGANIZER: u8 = 1;
