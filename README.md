# Linkup Email Reminder

A simple email notification script written in Rust for coordinating weekly group activities with turn-based organization.
Chose Rust to get familiar with the language and ecosystem.

## Features

- Sends bulk email reminders via Gmail SMTP
- Turn-based system where each person takes turns organizing activities
- Persistent turn state management with automatic cycling
- Interactive confirmation before sending emails
- Structured logging with configurable levels
- CSV-based recipient management

## Setup

### Required Files (Not in Git)

Create these files in the `resources/` directory:

**`resources/credentials.txt`**
```
your-gmail-app-password
```

**`resources/recipients.csv`**
```
email,turn,recipient_type
friend1@example.com,1,0
friend2@example.com,2,0
organizer@gmail.com,0,1
```
- `recipient_type: 0` = regular recipient
- `recipient_type: 1` = organizer (sends emails)

**`resources/email_template.txt`**
```
Your custom email message here.

This will be the body of your reminder email.
```

### Gmail Setup

1. Enable 2-factor authentication on your Gmail account
2. Generate an app password for the application
3. Use the app password in `credentials.txt`

## Usage

```bash
# Run with info logging
RUST_LOG=info cargo run -- send

# Run with debug logging
RUST_LOG=debug cargo run -- send
```

# TODO
- [X] Implement cargo like package structure
- [X] Implement clap cli
- [ ] Automation
- [ ] Better resource parsing
- [ ] Ideas
- [ ] Showtime scraper


## Project Structure

Following cargo's CLI architecture pattern:

```
src/
├── main.rs                    # Minimal entry point, sets up logging and delegates to CLI
├── cli/
│   ├── mod.rs                 # Main CLI entry point, argument parsing, command dispatch
│   ├── args.rs                # Shared argument definitions and utilities
│   └── commands/
│       ├── mod.rs             # Lists builtin commands (builtin() -> Vec<Command>)
│       ├── send.rs            # Send reminder emails (main functionality)
│       ├── list.rs            # List recipients and their turn information
│       ├── add/remove.rs      # TBD 
└── core/
    ├── mod.rs                 # Core module exports
    ├── context.rs             # Global application context
    ├── config.rs              # Configuration loading and management  
    ├── types.rs               # Data structures (Recipient, etc.)
    ├── errors.rs              # Custom error types and handling
    ├── email/
    │   ├── mod.rs            # Email module exports TBD
    │   ├── sender.rs         # SMTP email sending logic TBD
    │   └── template.rs       # Email template processing TBD
    └── storage/
        ├── mod.rs            # Storage module exports
        ├── recipients.rs     # CSV recipient management TBD
        ├── state.rs          # Turn state persistence
        └── files.rs          # General file I/O utilities
```


