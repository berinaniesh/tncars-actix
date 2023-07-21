use crate::misc::appstate::AppState;
use crate::misc::jwt::get_id_from_request;
use crate::models::comments::{AddComment, CommentDelete, CommentOut};
use crate::models::Response;
use actix_web::{delete, post, web, HttpRequest, HttpResponse};

#[post("/addcomment/{post_id}")]
pub async fn add_comment(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    form: web::Json<AddComment>,
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
    let q1 = sqlx::query_as!(CommentOut, "INSERT INTO comments (user_id, post_id, comment) VALUES ($1, $2, $3) RETURNING comments.id, comments.user_id, comments.post_id, comments.comment", user_id, post_id, &form.comment).fetch_one(&app_state.pool).await;
    if q1.is_err() {
        return HttpResponse::InternalServerError().json(Response {
            message: "Something went wrong, try again later".to_string(),
        });
    }
    return HttpResponse::Created().json(q1.unwrap());
}

#[delete("/deletecomment/{comment_id}")]
pub async fn delete_comment(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> HttpResponse {
    let comment_id = path.into_inner();
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
    let q1 = sqlx::query_as!(
        CommentDelete,
        "SELECT user_id FROM comments WHERE id=$1",
        comment_id
    )
    .fetch_one(&app_state.pool)
    .await;
    if q1.is_err() {
        return HttpResponse::BadRequest().json(Response {
            message: "The comment you requested does not exist".to_string(),
        });
    }
    let comment_user_id = q1.unwrap().user_id;
    if comment_user_id != user_id {
        return HttpResponse::Unauthorized().json(Response {
            message: "You cannot delete someone else's comment".to_string(),
        });
    }
    let q2 = sqlx::query!("DELETE FROM comments WHERE id=$1", comment_id)
        .execute(&app_state.pool)
        .await;
    if q2.is_ok() {
        return HttpResponse::Ok().json(Response {
            message: "Comment deleted".to_string(),
        });
    }
    return HttpResponse::InternalServerError().json(Response {
        message: "Something went wrong, try again later".to_string(),
    });
}
