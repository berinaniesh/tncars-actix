use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt;

#[derive(sqlx::Type)]
#[sqlx(type_name = "transmission_type")]
#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum TransmissionType {
    Manual,
    Automatic,
    NotApplicable,
}

impl fmt::Display for TransmissionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "fuel_type")]
#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum FuelType {
    Petrol,
    Diesel,
    CNG,
    Electric,
    Other,
}

impl fmt::Display for FuelType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CreatePost {
    pub title: String,
    pub brand: String,
    pub price: i32,
    pub model_year: i32,
    pub km_driven: i32,
    pub transmission: TransmissionType,
    pub fuel: FuelType,
    pub description: String,
    pub location: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UpdatedPost {
    pub title: String,
    pub user_id: i32,
    pub brand: String,
    pub price: i32,
    pub model_year: i32,
    pub km_driven: i32,
    pub transmission: TransmissionType,
    pub fuel: FuelType,
    pub description: String,
    pub location: String,
    pub is_sold: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PostOut {
    pub id: i32,
    pub title: String,
    pub user_id: i32,
    pub brand: String,
    pub post_pic: Option<String>,
    pub price: i32,
    pub model_year: i32,
    pub km_driven: i32,
    pub transmission: TransmissionType,
    pub fuel: FuelType,
    pub description: String,
    pub location: String,
    pub is_sold: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UpdatePost {
    pub title: Option<String>,
    pub brand: Option<String>,
    pub price: Option<i32>,
    pub model_year: Option<i32>,
    pub km_driven: Option<i32>,
    pub transmission: Option<TransmissionType>,
    pub fuel: Option<FuelType>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub is_sold: Option<bool>,
}
