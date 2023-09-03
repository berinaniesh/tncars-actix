use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SendMessage {
    pub to: i32,
    pub message: String,
}
