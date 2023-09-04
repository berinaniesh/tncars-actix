use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct AddComment {
    pub comment: String,
}

#[derive(Serialize, Debug)]
pub struct CommentOut {
    pub id: i32,
    pub user_id: i32,
    pub post_id: i32,
    pub comment: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct CommentUser {
    pub user_id: i32,
}
