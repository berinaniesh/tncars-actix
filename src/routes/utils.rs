use crate::misc::appstate::AppState;
use crate::misc::utils::is_available_username;
use crate::misc::validator::get_valid_username;
use crate::models::Response;
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
