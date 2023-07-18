pub mod hello;
pub mod users;
pub mod verify;

use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}
