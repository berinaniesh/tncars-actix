use crate::misc::appstate::AppState;
use crate::misc::jwt::get_id_from_request;
use crate::models::verify::{EmailVerOtp, EmailVerUrl};
use crate::models::Response;
use crate::routes::helper::make_email_verified;
use actix_web::{get, web, HttpRequest, HttpResponse};
use chrono::Utc;

#[get("/verify/email/otp/{var}")]
pub async fn email_otp(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    let otp = path.into_inner();
    let user_id_result = get_id_from_request(&req, &app_state);
    let user_id: i32;
    match user_id_result.await {
        Ok(id) => {user_id = id;},
        Err(e) => {
            return HttpResponse::Unauthorized().json(Response{
                message: e.to_string()
            });
        }
    }
    let verify_result = sqlx::query_as!(
        EmailVerOtp,
        "SELECT user_id, otp, expires_at FROM email_otp WHERE user_id=$1",
        user_id
    )
    .fetch_one(&app_state.pool)
    .await;
    if verify_result.is_err() {
        return HttpResponse::InternalServerError().json(Response {
            message: "Something went wrong, try again later".to_string(),
        });
    }
    let email_ver_otp = verify_result.unwrap();
    if email_ver_otp.expires_at < Utc::now() {
        return HttpResponse::BadRequest().json(Response {
            message: "OTP expired, get a new one".to_string(),
        });
    }
    if email_ver_otp.otp != otp {
        return HttpResponse::Unauthorized().json(Response {
            message: "Wrong OTP entered".to_string(),
        });
    }
    return make_email_verified(user_id, &app_state).await;
}

#[get("/verify/email/url/{var}")]
pub async fn email_url(app_state: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let url = path.into_inner();
    let verify_result = sqlx::query_as!(
        EmailVerUrl,
        "SELECT user_id, expires_at FROM email_otp WHERE verify_url=$1",
        url
    )
    .fetch_one(&app_state.pool)
    .await;
    if verify_result.is_err() {
        return HttpResponse::BadRequest().json(Response {
            message: "Wrong verification URL".to_string(),
        });
    }
    let email_ver_url = verify_result.unwrap();
    if email_ver_url.expires_at < Utc::now() {
        return HttpResponse::BadRequest().json(Response {
            message: "Verification URL expired, get a new one".to_string(),
        });
    }
    return make_email_verified(email_ver_url.user_id, &app_state).await;
}
