use crate::misc::appstate::AppState;
use crate::misc::jwt::get_id_from_request;
use crate::models::{posts::{CreatePost, CreatePostWithUserId, PostOut, UpdatePost}, Response};
use actix_web::{get, post, patch, web, HttpRequest, HttpResponse};
use crate::misc::utils::get_correct_post_form;
use crate::misc::utils::get_updated_post;

#[post("/posts")]
pub async fn create_post(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    user_form: web::Json<CreatePost>,
) -> HttpResponse {
    let user_id_result = get_id_from_request(&req);
    if user_id_result.is_err() {
        return HttpResponse::BadRequest().json(Response {
            message: "Invalid authorization headers".to_string(),
        });
    }
    let user_id = user_id_result.unwrap();
    let form = get_correct_post_form(user_form);
    let query = sqlx::query_as::<_, PostOut>(
        "INSERT INTO posts (title, user_id, brand, price, model_year, km_driven, transmission, fuel, description, location) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING id, title, user_id, brand, price, model_year, km_driven, transmission, fuel, description, location, is_sold, created_at, updated_at") 
        .bind(&form.title)
        .bind(user_id)
        .bind(&form.brand)
        .bind(&form.price)
        .bind(&form.model_year)
        .bind(&form.km_driven)
        .bind(form.transmission)
        .bind(form.fuel)
        .bind(&form.description)
        .bind(&form.location)
        .fetch_one(&app_state.pool)
        .await;

    if query.is_ok() {
        return HttpResponse::Created().json(query.unwrap());
    }
    return HttpResponse::InternalServerError().json(Response {
        message: "Something went wrong, try again later".to_string(),
    });
}

#[get("/posts/{id}")]
pub async fn get_post(app_state: web::Data<AppState>, path: web::Path<i32>) -> HttpResponse {
    let post_id = path.into_inner();
    let query = sqlx::query_as::<_, PostOut>(
        "SELECT id, title, user_id, brand, price, model_year, km_driven, transmission, fuel, description, location, is_sold, created_at, updated_at FROM posts where id=$1") 
        .bind(post_id)
        .fetch_one(&app_state.pool)
        .await;

    if query.is_ok() {
        return HttpResponse::Ok().json(query.unwrap());
    }
    return HttpResponse::BadRequest().json(Response {
        message: format!("Post with id: {} not found", post_id),
    });
}

#[patch("/posts/{id}")]
pub async fn update_post(
    req: HttpRequest,
    form: web::Json<UpdatePost>,
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> HttpResponse {
    let post_id = path.into_inner();
    let user_id_result = get_id_from_request(&req);
    if user_id_result.is_err() {
        return HttpResponse::BadRequest().json(Response {
            message: "Invalid authorization headers".to_string(),
        });
    }
    let q1 = sqlx::query_as::<_, CreatePostWithUserId>("SELECT title, user_id, brand, price, model_year, km_driven, transmission, fuel, description, location FROM posts WHERE id=$1").bind(post_id).fetch_one(&app_state.pool).await;
    if q1.is_err() {
        return HttpResponse::BadRequest().json(Response {
            message: format!("The requested post id: {} was not found in the database", post_id)
        });
    }
    let db_data = q1.unwrap();
    let user_id_db = db_data.user_id;
    let user_id_jwt: i32 = user_id_result.unwrap();
    if user_id_db != user_id_jwt {
        return HttpResponse::Unauthorized().json(Response {
            message: "You cannot modify someone else's post".to_string()
        });
    }
    let updated_post = get_updated_post(form, db_data);
    let q2 = sqlx::query_as::<_, PostOut>(
        "UPDATE posts set title=$1, brand=$2, price=$3, model_year=$4, km_driven=$5, transmission=$6, fuel=$7, description=$8, location=$9 where id=$10 RETURNING id, title, user_id, brand, price, model_year, km_driven, transmission, fuel, description, location, is_sold, created_at, updated_at")
        .bind(&updated_post.title)
        .bind(&updated_post.brand)
        .bind(&updated_post.price)
        .bind(&updated_post.model_year)
        .bind(&updated_post.km_driven)
        .bind(updated_post.transmission)
        .bind(updated_post.fuel)
        .bind(&updated_post.description)
        .bind(&updated_post.location)
        .bind(post_id)
        .fetch_one(&app_state.pool)
        .await;
    if q2.is_err() {
        return HttpResponse::InternalServerError().json(Response {
            message: "Something went wrong, try again later".to_string()
        });
    }
    let postout = q2.unwrap();
    return HttpResponse::Ok().json(postout);
}