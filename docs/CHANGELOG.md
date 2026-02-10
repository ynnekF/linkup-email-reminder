### Change Log

#### [2.0.1] - 2026-02-09

- Include changelog in email template
- Include date of, and time till next Wednesday
- Move email closer to after body

Dev notes
- New send command 'debug' flag for testing email templates without sending
- Check in `docs`, `resources` dir and include `resources/private` for sensitive info like email templates and changelog
- Refactor out threading context through storage module function calls

#### [2.0.0] - 2026-02-08

- Implemented a cli using clap
- Refactored to use a more modular architecture

#### [1.0.0] - 2026-02-08

- Initial release including ability to parse recipients CSV, generate email templates, and send emails via SMTP.
