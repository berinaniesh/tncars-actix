use actix_web::{web, get, post, HttpRequest, HttpResponse, Responder};
use crate::misc::appstate::AppState;
use crate::models::messages::{SendMessage};
use crate::models::Response;
use crate::misc::jwt::get_id_from_request;

#[post("/messages")]
async fn send_message(req: HttpRequest, app_state: web::Data<AppState>, form: web::Json<SendMessage>) -> HttpResponse {
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
    let _ = sqlx::query!("INSERT INTO messages (from_user, to_user, message) VALUES ($1, $2, $3)", user_id, &form.to, &form.message).execute(&app_state.pool).await.unwrap();
    let response = Response {message: String::from("Message sent")};
    return HttpResponse::Ok().json(response);
}
