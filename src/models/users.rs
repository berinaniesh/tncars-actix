use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

pub struct IdPassword {
    // struct to query as when checking for password and returning jwt.
    pub id: i32,
    pub password: String,
}

#[derive(Serialize)]
pub struct JWTResponse {
    pub jwt: String,
}

#[derive(Serialize)]
pub struct UserOut {
    pub id: i32,
    pub email: String,
    pub username: Option<String>,
    pub phone: Option<String>,
    pub bio: Option<String>,
    pub address: Option<String>,
    pub profile_pic_url: Option<String>,
    pub credits: i32,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct EmailOTP {
    pub id: i32,
    pub email: String,
    pub email_verified: bool,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub username: Option<String>,
    pub phone: Option<String>,
    pub bio: Option<String>,
    pub address: Option<String>,
}
