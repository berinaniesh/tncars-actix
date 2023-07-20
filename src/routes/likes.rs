use actix_web::{web, post, HttpRequest, HttpResponse};
use crate::misc::jwt::get_id_from_request;

#[post("/addlike/{id}")]
pub async fn add_like(req: HttpRequest, app_state: web::Data<AppState>, path: web::Path<i32>) -> HttpResponse {
    let user_id_result = get_id_from_request(&req, &app_state);
    let user_id: i32;
    match user_id_result.await {
        Ok(id) => {
            user_id = id;
        }
        Err(e) => {
            return HttpResponse::Unauthorized().json(Response {
                message: e.to_string(),
            });
        }
    }
}