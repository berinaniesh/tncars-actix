use crate::misc::appstate::AppState;
use crate::misc::jwt::get_id_from_request;
use crate::models::Response;
use actix_web::{post, web, HttpRequest, HttpResponse};

#[post("/like/{id}")]
pub async fn add_like(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> HttpResponse {
    let post_id = path.into_inner();
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
    // Assumes UNIQUE(user_id, post_id) is set.
    let q1 = sqlx::query!(
        "INSERT INTO likes (user_id, post_id) VALUES ($1, $2)",
        user_id,
        post_id
    )
    .execute(&app_state.pool)
    .await;
    if q1.is_ok() {
        return HttpResponse::Created().json(Response {
            message: "Like added".to_string(),
        });
    }
    let q2 = sqlx::query!(
        "DELETE FROM likes WHERE user_id=$1 AND post_id=$2",
        user_id,
        post_id
    )
    .execute(&app_state.pool)
    .await;
    if q2.is_ok() {
        return HttpResponse::Ok().json(Response {
            message: "Like removed".to_string(),
        });
    }
    return HttpResponse::InternalServerError().json(Response {
        message: "Something went wrong, try again later".to_string(),
    });
}
