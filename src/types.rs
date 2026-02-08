use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Recipient {
    pub email: String,
    pub turn: u32,
    pub recipient_type: u8,
}
