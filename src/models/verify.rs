//use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

pub struct EmailVerOtp {
    pub user_id: i32,
    pub otp: String,
    pub expires_at: DateTime<Utc>,
}

pub struct EmailVerUrl {
    pub user_id: i32,
    pub expires_at: DateTime<Utc>,
}