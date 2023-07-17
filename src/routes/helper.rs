use actix_web::web;
use chrono::{Utc, Duration};
use crate::misc::constants::OTP_EXPIRY;
use crate::misc::appstate::AppState;
use crate::misc::email::send_email;
use crate::misc::utils::{generate_otp, generate_verify_url};

pub async fn create_otp_and_and_send_email(user_id: i32, email: String, app_state: &web::Data<AppState>) -> bool  {
    let otp = generate_otp();
    let verify_url = generate_verify_url();
    let expiry = Utc::now() + Duration::seconds(OTP_EXPIRY);
    let _delete_query = sqlx::query!("DELETE FROM email_otp where id=$1", user_id).execute(&app_state.pool).await.unwrap();
    let _insert_query = sqlx::query!("INSERT INTO email_otp (user_id, otp, verify_url, expiry) values ($1, $2, $3, $4)", user_id, otp, verify_url, expiry).execute(&app_state.pool).await.unwrap();
    return send_email(email, otp, verify_url);
}