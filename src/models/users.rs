use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub credential: String,
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
    pub username: String,
    pub phone: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub bio: Option<String>,
    pub address: Option<String>,
    pub profile_pic: Option<String>,
    pub credits: i32,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub email_public: bool,
    pub phone_public: bool,
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
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub bio: Option<String>,
    pub address: Option<String>,
    pub email_public: Option<bool>,
    pub phone_public: Option<bool>,
}

#[derive(Serialize)]
pub struct UserOutPublic {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub bio: Option<String>,
    pub address: Option<String>,
    pub profile_pic: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl UserOut {
    pub fn get_public_user(&self) -> UserOutPublic {
        let email;
        if self.email_public {
            email = Some(self.email.clone());
        } else {
            email = None;
        }
        let phone;
        if self.phone_public {
            phone = self.phone.clone()
        } else {
            phone = None
        }
        return UserOutPublic {
            id: self.id,
            username: self.username.clone(),
            email: email,
            phone: phone,
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            bio: self.bio.clone(),
            address: self.address.clone(),
            profile_pic: self.profile_pic.clone(),
            created_at: self.created_at.clone(),
        };
    }
}
