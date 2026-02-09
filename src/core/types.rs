use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Recipient {
    pub email: String,
    pub turn: u32,
    pub recipient_type: u8,
}
