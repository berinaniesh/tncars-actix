use crate::misc::appstate::AppState;
use crate::misc::jwt::get_id_from_request;
use crate::misc::utils::get_id;
use crate::models::follows::UserIds;
use crate::models::Response;
use actix_web::{get, post, web, HttpRequest, HttpResponse};

#[post("/follow/user/{id}")]
pub async fn follow_user(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    // Needs change according to the new error handling
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
    let username_to_follow = path.into_inner();
    let to_follow_opt = get_id(&username_to_follow);
    if to_follow_opt.is_some() {
        let to_follow = to_follow_opt.unwrap();
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
    }
    let q3 = sqlx::query!("INSERT INTO follows (from_user, to_user) VALUES ($1, (SELECT users.id FROM users WHERE username=$2))", from_follow, username_to_follow).execute(&app_state.pool).await;
    if q3.is_ok() {
        return HttpResponse::Created().json(Response {
            message: "Follow added".to_string(),
        });
    }
    let q4 = sqlx::query!("DELETE FROM follows WHERE from_user=$1 AND to_user=(SELECT users.id FROM users WHERE username=$2)", from_follow, username_to_follow).execute(&app_state.pool).await;
    if q4.is_ok() {
        return HttpResponse::Ok().json(Response {
            message: "Follow removed".to_string(),
        });
    }
    return HttpResponse::InternalServerError().json(Response {
        message: "Something went wrong, try again later".to_string(),
    });
}

#[get("/following/{id}")]
pub async fn get_following(
    path: web::Path<String>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    // Needs change according to the new error handling
    let username = path.into_inner();
    let user_id_opt = get_id(&username);
    if user_id_opt.is_some() {
        let user_id = user_id_opt.unwrap();
        let q1 = sqlx::query_as!(
            UserIds,
            "SELECT to_user AS user_id FROM follows WHERE from_user=$1",
            user_id
        )
        .fetch_all(&app_state.pool)
        .await;
        if q1.is_ok() {
            let v = q1.unwrap();
            let mut ans_vec = Vec::new();
            for i in v {
                ans_vec.push(i.user_id)
            }
            return HttpResponse::Ok().json(ans_vec);
        }
    }
    let q2 = sqlx::query_as!(UserIds, "SELECT to_user AS user_id from follows WHERE from_user=(SELECT users.id FROM users WHERE username=$1)", username).fetch_all(&app_state.pool).await;
    if q2.is_ok() {
        let v = q2.unwrap();
        let mut ans_vec = Vec::new();
        for i in v {
            ans_vec.push(i.user_id);
        }
        return HttpResponse::Ok().json(ans_vec);
    }
    return HttpResponse::BadRequest().json(Response {
        message: format!("The requested user: {} cannot be found", username),
    });
}

#[get("/followedby/{id}")]
pub async fn get_followed_by(
    path: web::Path<String>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    // Needs change according to new error handling
    let username = path.into_inner();
    let user_id_opt = get_id(&username);
    if user_id_opt.is_some() {
        let user_id = user_id_opt.unwrap();
        let q1 = sqlx::query_as!(
            UserIds,
            "SELECT from_user AS user_id FROM follows WHERE to_user=$1",
            user_id
        )
        .fetch_all(&app_state.pool)
        .await;
        if q1.is_ok() {
            let v = q1.unwrap();
            let mut ans_vec = Vec::new();
            for i in v {
                ans_vec.push(i.user_id)
            }
            return HttpResponse::Ok().json(ans_vec);
        }
    }
    let q2 = sqlx::query_as!(UserIds, "SELECT from_user AS user_id from follows WHERE to_user=(SELECT users.id FROM users WHERE username=$1)", username).fetch_all(&app_state.pool).await;
    if q2.is_ok() {
        let v = q2.unwrap();
        let mut ans_vec = Vec::new();
        for i in v {
            ans_vec.push(i.user_id);
        }
        return HttpResponse::Ok().json(ans_vec);
    }
    return HttpResponse::BadRequest().json(Response {
        message: format!("The requested user: {} cannot be found", username),
    });
}
