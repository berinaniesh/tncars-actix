use crate::misc::{appstate::AppState, validator::validate_email};
use crate::misc::utils::is_available_username;
use crate::misc::validator::get_valid_username;
use crate::models::Response;
use crate::misc::utils::get_id;
use crate::models::utils::IdUsernameEmail;
use actix_web::{get, web, HttpResponse};

#[get("/utils/usernameavailable/{id}")]
pub async fn is_username_available(
    path: web::Path<String>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let requested_username = path.into_inner();
    let to_check_result = get_valid_username(&requested_username);
    if to_check_result.is_none() {
        return HttpResponse::BadRequest().json(Response {
            message: "Bad username requested".to_string(),
        });
    }
    let to_check = to_check_result.unwrap();
    if is_available_username(&to_check, &app_state).await {
        return HttpResponse::Ok().json(Response {
            message: "Username available".to_string(),
        });
    }
    return HttpResponse::BadRequest().json(Response {
        message: "Username taken".to_string(),
    });
}

#[get("/utils/idusernameemail/{id}")]
pub async fn get_id_username_email(path: web::Path<String>, app_state: web::Data<AppState>) -> HttpResponse {
    let url_parameter = path.into_inner();
    let id_int_opt = get_id(&url_parameter);
    if id_int_opt.is_some() {
        let id = id_int_opt.unwrap();
        let q1 = sqlx::query_as!(IdUsernameEmail, "SELECT id, username, email FROM users WHERE id=$1", id).fetch_one(&app_state.pool).await;
        if q1.is_ok() {
            return HttpResponse::Ok().json(q1.unwrap());
        }
    }
    if validate_email(&url_parameter) {
        let q2 = sqlx::query_as!(IdUsernameEmail, "SELECT id, username, email FROM users WHERE email=$1", &url_parameter).fetch_one(&app_state.pool).await;
        if q2.is_ok() {
            return HttpResponse::Ok().json(q2.unwrap());
        }
    }
    let username_res = get_valid_username(&url_parameter);
    if username_res.is_some() {
        let username = username_res.unwrap();
        let q3 = sqlx::query_as!(IdUsernameEmail, "SELECT id, username, email FROM users WHERE username=$1", &username).fetch_one(&app_state.pool).await;
        if q3.is_ok() {
            return HttpResponse::Ok().json(q3.unwrap());
        }
    }
    return HttpResponse::NotFound().json(Response{
        message: format!("User {} not found", url_parameter)
    });
}