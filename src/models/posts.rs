use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt;

#[derive(sqlx::Type)]
#[sqlx(type_name = "transmission_type")]
#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum TransmissionType {
    Manual,
    Automatic,
    NotApplicable
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

#[derive(Serialize, Deserialize, FromRow)]
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