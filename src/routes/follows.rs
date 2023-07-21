use crate::misc::appstate::AppState;
use crate::misc::jwt::get_id_from_request;
use crate::models::Response;
use actix_web::{post, web, HttpRequest, HttpResponse};

#[post("/follow/user/{id}")]
pub async fn follow_user_by_id(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> HttpResponse {
    let to_follow = path.into_inner();
    let user_id_result = get_id_from_request(&req, &app_state);
    let from_follow: i32;
    match user_id_result.await {
        Ok(id) => {
            from_follow = id;
        }
        Err(e) => {
            return HttpResponse::Unauthorized().json(Response {
                message: e.to_string(),
            });
        }
    }
    let q1 = sqlx::query!(
        "INSERT INTO follows (from_user, to_user) VALUES ($1, $2)",
        from_follow,
        to_follow
    )
    .execute(&app_state.pool)
    .await;
    if q1.is_ok() {
        return HttpResponse::Created().json(Response {
            message: "Follow added".to_string(),
        });
    }
    let q2 = sqlx::query!(
        "DELETE FROM follows WHERE from_user=$1 AND to_user=$2",
        from_follow,
        to_follow
    )
    .execute(&app_state.pool)
    .await;
    if q2.is_ok() {
        return HttpResponse::Ok().json(Response {
            message: "Follow removed".to_string(),
        });
    }
    return HttpResponse::InternalServerError().json(Response {
        message: "Something went wrong, try again later".to_string(),
    });
}

#[post("/follow/username/{username}")]
pub async fn follow_user_by_username(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    let username_to_follow = path.into_inner();
    let user_id_result = get_id_from_request(&req, &app_state);
    let from_follow: i32;
    match user_id_result.await {
        Ok(id) => {
            from_follow = id;
        }
        Err(e) => {
            return HttpResponse::Unauthorized().json(Response {
                message: e.to_string(),
            });
        }
    }
    let q1 = sqlx::query!("INSERT INTO follows (from_user, to_user) VALUES ($1, (SELECT users.id FROM users WHERE username=$2))", from_follow, username_to_follow).execute(&app_state.pool).await;
    if q1.is_ok() {
        return HttpResponse::Created().json(Response {
            message: "Follow added".to_string(),
        });
    }
    let q2 = sqlx::query!("DELETE FROM follows WHERE from_user=$1 AND to_user=(SELECT users.id FROM users WHERE username=$2)", from_follow, username_to_follow).execute(&app_state.pool).await;
    if q2.is_ok() {
        return HttpResponse::Ok().json(Response {
            message: "Follow removed".to_string(),
        });
    }
    return HttpResponse::InternalServerError().json(Response {
        message: "Something went wrong, try again later".to_string(),
    });
}
