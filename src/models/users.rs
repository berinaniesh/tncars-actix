use serde::{Deserialize};


#[derive(Deserialize, Debug)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
}