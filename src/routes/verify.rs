use actix_web::{get, web, HttpRequest, HttpResponse};
use crate::misc::appstate::AppState;
use crate::models::Response;
use crate::models::verify::{EmailVerOtp, EmailVerUrl};
use crate::misc::jwt::get_id_from_request;
use crate::routes::helper::make_email_verified;
use chrono::Utc;

#[get("/verify/email/otp/{var}")]
pub async fn email_otp(req: HttpRequest, app_state: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let otp = path.into_inner();
    let user_id_result = get_id_from_request(&req);
    if user_id_result.is_err() {
        return HttpResponse::BadRequest().json(Response{message: "Invalid authorization headers".to_string()});
    }
    let user_id = user_id_result.unwrap();
    let verify_result = sqlx::query_as!(EmailVerOtp, "SELECT user_id, otp, expires_at FROM email_otp WHERE user_id=$1", user_id).fetch_one(&app_state.pool).await;
    if verify_result.is_err() {
        return HttpResponse::InternalServerError().json(Response{message: "Something went wrong, try again later".to_string()});
    }
    let email_ver_otp = verify_result.unwrap();
    if email_ver_otp.expires_at < Utc::now() {
        return HttpResponse::BadRequest().json(Response{message: "OTP expired, get a new one".to_string()})
    }
    if email_ver_otp.otp != otp {
        return HttpResponse::Unauthorized().json(Response{message: "Wrong OTP entered".to_string()});
    }
    return make_email_verified(user_id, &app_state).await;
}

#[get("/verify/email/url/{var}")]
pub async fn email_url(req: HttpRequest, app_state: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let url = path.into_inner();
    return HttpResponse::Ok().json(Response{message: url});
}