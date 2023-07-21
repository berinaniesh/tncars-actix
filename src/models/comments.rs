use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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
    pub created_at: DateTime<Utc>
}

pub struct CommentDelete {
    pub user_id: i32,
}
