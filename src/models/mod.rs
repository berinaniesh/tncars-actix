pub mod comments;
pub mod follows;
pub mod hello;
pub mod posts;
pub mod users;
pub mod utils;
pub mod verify;
pub mod messages;

use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}
