use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct IdUsernameEmail {
    pub id: i32,
    pub username: String,
    pub email: String,
}
