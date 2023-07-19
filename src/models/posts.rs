use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub price: i32,
    pub model_yer: i32,
    pub km_driven: i32,
    pub description: String,
    pub location: String,
}