use rand::distributions::{Alphanumeric, DistString};
use actix_web::web;
use crate::models::posts::{CreatePost, CreatePostWithUserId, UpdatePost, TransmissionType, FuelType};
use heck::AsTitleCase;
use crate::misc::validator::validate_year;

pub fn generate_otp() -> String {
    let string = Alphanumeric
        .sample_string(&mut rand::thread_rng(), 7)
        .to_uppercase();
    return string;
}

pub fn generate_verify_url() -> String {
    let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 50);
    return string;
}

pub fn make_first_letter_capital(s: String) -> String {
    if s.len() == 0 || !s.is_ascii() || &s[0..1] == &s[0..1].to_uppercase() {
        return s;
    }
    return String::from(s[0..1].to_uppercase() + &s[1..]);
}

pub fn get_correct_post_form(form: web::Json<CreatePost>) -> CreatePost {
    let title = make_first_letter_capital(form.title.trim().to_string());
    let brand = AsTitleCase(form.brand.trim()).to_string();
    let price = form.price;
    let model_year = validate_year(form.model_year);
    let km_driven = form.km_driven;
    let transmission = form.transmission;
    let fuel = form.fuel;
    let description = form.description.trim().to_string();
    let location = AsTitleCase(form.location.trim()).to_string();

    return CreatePost{title: title, brand: brand, price: price, model_year: model_year, km_driven: km_driven, transmission: transmission, fuel: fuel, description: description, location: location};
}

pub fn get_updated_post(form: web::Json<UpdatePost>, db_data: CreatePostWithUserId) -> CreatePost {
    let title: String;
    let brand: String;
    let price: i32;
    let model_year: i32;
    let km_driven: i32;
    let transmission: TransmissionType;
    let fuel: FuelType;
    let description: String;
    let location: String;

    if form.title.is_some() {
        title = make_first_letter_capital(form.title.as_ref().unwrap().trim().to_string());
    } else {
        title = db_data.title;
    }

    if form.brand.is_some() {
        brand = AsTitleCase(form.brand.as_ref().unwrap().trim()).to_string();
    } else {
        brand = db_data.brand;
    }

    if form.price.is_some() {
        price = form.price.unwrap()
    } else {
        price = db_data.price;
    }

    if form.model_year.is_some() {
        model_year = validate_year(form.model_year.unwrap());
    } else {
        model_year = db_data.model_year;
    }

    if form.km_driven.is_some() {
        km_driven = form.km_driven.unwrap();
    } else {
        km_driven = db_data.km_driven;
    }

    if form.transmission.is_some() {
        transmission = form.transmission.clone().unwrap();
    } else {
        transmission = db_data.transmission;
    }

    if form.fuel.is_some() {
        fuel = form.fuel.unwrap();
    } else {
        fuel = db_data.fuel;
    }

    if form.description.is_some() {
        description = form.description.as_ref().unwrap().trim().to_string();
    } else {
        description = db_data.description;
    }

    if form.location.is_some() {
        location = AsTitleCase(form.location.as_ref().unwrap().trim()).to_string();
    } else {
        location = db_data.location;
    }

    return CreatePost{title: title, brand: brand, price: price, model_year: model_year, km_driven: km_driven, transmission: transmission, fuel: fuel, description: description, location: location};
}