use serde::{Serialize, Deserialize};
use jsonwebtoken;
use chrono::Utc;
use anyhow::Error;
use actix_web::HttpRequest;

const JWT_EXPIRY: i64 = 604_800; // Expiry time for a JWT - 604,800 seconds = 1 week. 

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    user_id: i32, // User Id - Primary Key of users table
    exp: i64, // Unix timestamp in seconds (UTC)
}

impl Claims {
    fn new(id: i32) -> Self {
        let current_timestamp = Utc::now().timestamp();
        let expiry = current_timestamp + JWT_EXPIRY;
        return Claims {
            user_id: id,
            exp: expiry,
        };
    }
}

pub fn generate_token(id: i32) -> Result<String, Error> {
    let claim = Claims::new(id);
    let secret = dotenvy::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claim,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
    )?;
    return Ok(token);
}

fn get_id_from_token(token: String) -> Result<i32, Error> {
    let secret = dotenvy::var("JWT_SECRET").expect("JWT_SECRET must be set");
    dbg!(&secret);
    let token_data_result = jsonwebtoken::decode::<Claims>(
        &token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    );
    let token_data = token_data_result.unwrap();
    dbg!(&token_data);
    if token_data.claims.exp < Utc::now().timestamp() {
        return Err(Error::msg("Token Expired"));
    }
    return Ok(token_data.claims.user_id);
}

pub fn get_id_from_request(req: &HttpRequest) -> Result<i32, Error> {
    let authorization_header = req.headers().get("authorization");
    if authorization_header.is_none() {
        return Err(Error::msg("Authorization token required"))
    }    
    let authorization = String::from(authorization_header.unwrap().to_str().unwrap());
    let token_type = &authorization[0..6];
    if token_type != "Bearer" {
        return Err(Error::msg("Authorization token type should be bearer"));
    }
    let token = &authorization[7..];
    let user_id_result = get_id_from_token(token.to_string());
    if user_id_result.is_err() {
        return Err(Error::msg("Bad Token"));
    }
    let user_id = user_id_result.unwrap();
    return Ok(user_id);
}
