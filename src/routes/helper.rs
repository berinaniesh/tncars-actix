use crate::misc::appstate::AppState;
use crate::misc::constants::OTP_EXPIRY;
use crate::misc::email::send_email;
use crate::misc::utils::{generate_otp, generate_verify_url};
use crate::misc::utils::{validate_email, validate_phone};
use crate::models::users::{UpdateUser, UserOut};
use crate::models::Response;
use actix_web::{web, HttpResponse};
use chrono::{Duration, Utc};

pub async fn create_otp_and_and_send_email(
    user_id: i32,
    email: String,
    app_state: &web::Data<AppState>,
) -> bool {
    let otp = generate_otp();
    let verify_url = generate_verify_url();
    let expiry = Utc::now() + Duration::seconds(OTP_EXPIRY);
    let _delete_query = sqlx::query!("DELETE FROM email_otp where id=$1", user_id)
        .execute(&app_state.pool)
        .await
        .unwrap();
    let _insert_query = sqlx::query!(
        "INSERT INTO email_otp (user_id, otp, verify_url, expires_at) values ($1, $2, $3, $4)",
        user_id,
        otp,
        verify_url,
        expiry
    )
    .execute(&app_state.pool)
    .await
    .unwrap();
    return send_email(email, otp, verify_url);
}

pub async fn make_email_verified(user_id: i32, app_state: &web::Data<AppState>) -> HttpResponse {
    let update_query = sqlx::query!("UPDATE users set email_verified='t' where id=$1", user_id)
        .execute(&app_state.pool)
        .await;
    let delete_query = sqlx::query!("DELETE from email_otp where user_id=$1", user_id)
        .execute(&app_state.pool)
        .await;
    if update_query.is_err() || delete_query.is_err() {
        return HttpResponse::InternalServerError().json(Response {
            message: "Something went wrong, try again later".to_string(),
        });
    }
    return HttpResponse::Ok().json(Response {
        message: "Email successfully verified".to_string(),
    });
}

pub async fn get_updated_user(
    user_id: i32,
    form: &web::Json<UpdateUser>,
    app_state: &web::Data<AppState>,
) -> UserOut {
    let user_result = sqlx::query_as!(UserOut, "SELECT email, username, phone, bio, address, profile_pic_url, credits, email_verified, phone_verified, is_active, created_at, updated_at FROM users WHERE id=$1", user_id).fetch_one(&app_state.pool).await;
    let mut user_out = user_result.unwrap();

    if form.email.is_some() {
        let form_email = form.email.as_ref().unwrap();
        if validate_email(&form_email) {
            if form_email.as_str() != user_out.email.as_str() {
                user_out.email = form_email.to_string();
                user_out.email_verified = false;
            }
        }
    }

    if form.phone.is_some() {
        let form_phone = form.phone.as_ref().unwrap();
        if validate_phone(form_phone.to_string()) {
            if user_out.phone.is_none() {
                user_out.phone = Some(form_phone.to_string());
                user_out.phone_verified = false;
            } else {
                if form_phone.as_str() != user_out.phone.as_ref().unwrap().as_str() {
                    user_out.phone = Some(form_phone.as_str().to_string());
                    user_out.phone_verified = false;
                }
            }
        }
    }

    if form.username.is_some() {
        user_out.username = Some(form.username.as_ref().unwrap().to_string());
    }

    if form.bio.is_some() {
        user_out.bio = Some(form.bio.as_ref().unwrap().to_string());
    }

    if form.address.is_some() {
        user_out.address = Some(form.address.as_ref().unwrap().to_string());
    }

    return user_out;
}
