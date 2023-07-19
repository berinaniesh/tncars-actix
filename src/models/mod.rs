pub mod hello;
pub mod users;
pub mod verify;
pub mod posts;

use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}
