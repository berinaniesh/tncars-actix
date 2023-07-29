use crate::misc::appstate::AppState;
use crate::misc::constants::OTP_EXPIRY;
use crate::misc::email::send_email;
use crate::misc::utils::{generate_otp, generate_verify_url};
use crate::misc::utils::{is_available_username, make_first_letter_capital};
use crate::misc::validator::get_valid_username;
use crate::misc::validator::{validate_email, validate_phone};
use crate::models::users::{UpdateUser, UserOut};
use crate::models::Response;
use actix_web::{web, HttpResponse};
use chrono::{Duration, Utc};

pub async fn create_otp_and_send_email(
    user_id: i32,
    email: String,
    app_state: &web::Data<AppState>,
) -> bool {
    let otp = generate_otp();
    let verify_url = generate_verify_url();
    let email_subject = String::from("Verify your account");
    let email_body = format!("The OTP to verify your account is {}.\nYou can also verify your account by clicking the link below.\nhttps://tncars.pp.ua/verify/url/{}.\nThe OTP and the link are valid for the next 15 minutes\nRegards,\ntncars.pp.ua", otp, verify_url);
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
    return send_email(email, otp, verify_url, email_subject, email_body);
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
    let user_result = sqlx::query_as!(UserOut, "SELECT id, email, username, phone, first_name, last_name, bio, address, profile_pic, credits, email_verified, phone_verified, email_public, phone_public, is_active, created_at, updated_at FROM users WHERE id=$1", user_id).fetch_one(&app_state.pool).await;
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

    if form.username.is_some() {
        let form_username = form.username.as_ref().unwrap();
        let proper_form_username = get_valid_username(&form_username);
        if proper_form_username.is_some() {
            let pfu = proper_form_username.unwrap();
            if is_available_username(&pfu, &app_state).await {
                user_out.username = pfu;
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
    if form.first_name.is_some() {
        user_out.first_name = Some(make_first_letter_capital(
            form.first_name.as_ref().unwrap().trim().to_string(),
        ));
    }
    if form.last_name.is_some() {
        user_out.last_name = Some(make_first_letter_capital(
            form.last_name.as_ref().unwrap().trim().to_string(),
        ))
    }
    if form.email_public.is_some() {
        user_out.email_public = form.email_public.unwrap();
    }
    if form.phone_public.is_some() {
        user_out.phone_public = form.phone_public.unwrap();
    }
    if form.bio.is_some() {
        user_out.bio = Some(form.bio.as_ref().unwrap().to_string());
    }

    if form.address.is_some() {
        user_out.address = Some(form.address.as_ref().unwrap().to_string());
    }

    return user_out;
}

pub async fn forgot_password_email(email: &String, app_state: &web::Data<AppState>) -> HttpResponse {
    let otp = generate_otp();
    let verify_url = generate_verify_url();
    let email_subject = String::from("Reset your password");
    let email_body = format!("Someone requested a password reset for your account. If that was not you, you can safely ignore this email.\nThe OTP to validate your identity is {}.\nYou can also reset your password following the link below.\nhttps://tncars.pp.ua/changepassword/url/{}.\nThe OTP and the link are valid for the next 15 minutes\nRegards,\ntncars.pp.ua", otp, verify_url);
    let expiry = Utc::now() + Duration::seconds(OTP_EXPIRY);
    let _delete_query = sqlx::query!("DELETE FROM forgot_password_email WHERE user_id=(SELECT id FROM users WHERE email=$1)", email)
        .execute(&app_state.pool)
        .await
        .unwrap();
    let _insert_query = sqlx::query!(
        "INSERT INTO forgot_password_email (user_id, otp, verify_url, expires_at) values ((SELECT id FROM users WHERE email=$1), $2, $3, $4)",
        email,
        otp,
        verify_url,
        expiry
    )
    .execute(&app_state.pool)
    .await
    .unwrap();
    let ans = send_email(email.to_string(), otp, verify_url, email_subject, email_body);
    return HttpResponse::Ok().json("Ok!");
}