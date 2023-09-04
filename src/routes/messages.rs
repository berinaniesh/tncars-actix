use crate::error::AppError;
use crate::misc::appstate::AppState;
use crate::misc::jwt::get_id_from_request;
use crate::models::messages::SendMessage;
use crate::models::Response;
use actix_web::{post, web, HttpRequest, HttpResponse};

#[post("/messages")]
async fn send_message(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    form: web::Json<SendMessage>,
) -> Result<HttpResponse, AppError> {
    let user_id_result = get_id_from_request(&req, &app_state);
    let user_id: i32;
    match user_id_result.await {
        Ok(id) => {
            user_id = id;
        }
        Err(e) => {
            return Ok(HttpResponse::Unauthorized().json(Response {
                message: e.to_string(),
            }));
        }
    }
    let _ = sqlx::query!(
        "INSERT INTO messages (from_user, to_user, message) VALUES ($1, $2, $3)",
        user_id,
        &form.to,
        &form.message
    )
    .execute(&app_state.pool)
    .await?;
    let response = Response {
        message: String::from("Message sent"),
    };
    return Ok(HttpResponse::Ok().json(response));
}
