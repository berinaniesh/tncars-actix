pub mod hello;
pub mod users;

use serde::Serialize;

#[derive(Serialize)]
pub struct Response{
    pub message: String
}