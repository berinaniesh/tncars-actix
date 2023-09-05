use crate::error::AppError;
use crate::misc::appstate::AppState;
use crate::misc::jwt::get_id_from_request;
use crate::misc::utils::get_correct_post_form;
use crate::misc::utils::get_updated_post;
use crate::models::comments::CommentOut;
use crate::models::posts::{CreatePost, ImagesOut, PostImg, PostOut, UpdatePost, UpdatedPost};
use crate::models::posts::{FuelType, TransmissionType};
use crate::models::Response;
use actix_web::{delete, get, patch, post, web, HttpRequest, HttpResponse};

#[post("/posts")]
pub async fn create_post(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    user_form: web::Json<CreatePost>,
) -> Result<HttpResponse, AppError> {
    let user_id_result = get_id_from_request(&req, &app_state).await;
    let user_id = user_id_result.unwrap();
    let form = get_correct_post_form(user_form);
    let query = sqlx::query_as!(PostOut,
        r#"
        INSERT INTO posts
        (title, user_id, brand, price, model_year, km_driven, transmission, fuel, description, location)
        values
        ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING
        id, title, user_id, brand, post_pic, price, model_year, km_driven, transmission as "transmission: _", fuel as "fuel: _", description, location, is_sold, created_at, updated_at
        "#,
        &form.title,
        user_id,
        &form.brand,
        &form.price,
        &form.model_year,
        &form.km_driven,
        form.transmission as TransmissionType,
        form.fuel as FuelType,
        &form.description,
        &form.location)
        .fetch_one(&app_state.pool)
        .await?;

    return Ok(HttpResponse::Created().json(query));
}

#[get("/posts/{id}")]
pub async fn get_post(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    let query = sqlx::query_as!(PostOut,
        r#"
        SELECT id, title, user_id, brand, post_pic, price, model_year, km_driven, transmission as "transmission: _", fuel as "fuel: _", description, location, is_sold, created_at, updated_at FROM posts where id=$1
        "#, post_id)
        .fetch_one(&app_state.pool)
        .await?;

    return Ok(HttpResponse::Ok().json(query));
}

#[patch("/posts/{id}")]
pub async fn update_post(
    req: HttpRequest,
    form: web::Json<UpdatePost>,
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
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
    let db_data = sqlx::query_as!(UpdatedPost,
        r#"
        SELECT
        title, user_id, brand, price, model_year, km_driven, transmission as "transmission: _", fuel as "fuel: _", description, location, is_sold
        FROM posts WHERE id=$1
        "#, post_id)
        .fetch_one(&app_state.pool).await?;

    let user_id_db = db_data.user_id;
    let user_id_jwt: i32 = user_id;
    if user_id_db != user_id_jwt {
        return Ok(HttpResponse::Unauthorized().json(Response {
            message: "You cannot modify someone else's post".to_string(),
        }));
    }
    let updated_post = get_updated_post(form, db_data);
    let postout = sqlx::query_as!(PostOut,
        r#"
        UPDATE posts set
        title=$1, brand=$2, price=$3, model_year=$4, km_driven=$5, transmission=$6, fuel=$7, description=$8, location=$9, is_sold=$10
        where id=$11
        RETURNING id, title, user_id, brand, post_pic, price, model_year, km_driven, transmission as "transmission: _", fuel as "fuel: _", description, location, is_sold, created_at, updated_at
        "#,
        &updated_post.title,
        &updated_post.brand,
        &updated_post.price,
        &updated_post.model_year,
        &updated_post.km_driven,
        updated_post.transmission as TransmissionType,
        updated_post.fuel as FuelType,
        &updated_post.description,
        &updated_post.location,
        &updated_post.is_sold,
        post_id)
        .fetch_one(&app_state.pool)
        .await?;
    return Ok(HttpResponse::Ok().json(postout));
}

#[delete("/posts/{id}")]
pub async fn delete_post(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
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

    let db_user = sqlx::query!(
        r#"
        SELECT user_id FROM posts WHERE id=$1
        "#,
        post_id
    )
    .fetch_one(&app_state.pool)
    .await?;

    let req_user = user_id;
    if req_user != db_user.user_id {
        return Ok(HttpResponse::Unauthorized().json(Response {
            message: "You cannot delete someone else's post".to_string(),
        }));
    }
    let _delete_req = sqlx::query!("DELETE FROM posts WHERE id=$1", post_id)
        .execute(&app_state.pool)
        .await?;
    return Ok(HttpResponse::Ok().json(Response {
        message: "Post deleted successfully".to_string(),
    }));
}

#[get("/posts/{post_id}/comments")]
pub async fn get_comments(path: web::Path<i32>, app_state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    let q1 = sqlx::query_as!(
        CommentOut,
        "SELECT * FROM comments WHERE post_id=$1",
        post_id
    )
    .fetch_all(&app_state.pool)
    .await?;
    return Ok(HttpResponse::Ok().json(q1));
}

#[get("/posts/{post_id}/images")]
pub async fn get_post_images(path: web::Path<i32>, app_state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    let q1 = sqlx::query_as!(
        ImagesOut,
        "SELECT id, image_url, created_at FROM posts_images WHERE post_id=$1",
        post_id
    )
    .fetch_all(&app_state.pool)
    .await?;
    return Ok(HttpResponse::Ok().json(q1));
}

#[delete("/posts/{post_id}/postpic")]
pub async fn delete_post_primary_image(
    req: HttpRequest,
    path: web::Path<i32>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
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
    let post_img = sqlx::query_as!(
        PostImg,
        "SELECT user_id, post_pic FROM posts WHERE id=$1",
        post_id
    )
    .fetch_one(&app_state.pool)
    .await?;
    if user_id != post_img.user_id {
        return Ok(HttpResponse::Unauthorized().json(Response {
            message: "You cannot delete someone else's posts's pic".to_string(),
        }));
    }
    let post_pic_opt = post_img.post_pic;
    if post_pic_opt.is_none() {
        return Ok(HttpResponse::NotAcceptable().json(Response {
            message: "The post does not have a primary pic".to_string(),
        }));
    }
    let post_pic_fname = post_pic_opt.unwrap();
    let del_result = tokio::fs::remove_file(format!("upload/{}", post_pic_fname)).await;
    if del_result.is_err() {
        println!("File {} not found in HDD", post_pic_fname);
    }
    let _set_null_query = sqlx::query!("UPDATE posts SET post_pic=NULL WHERE id=$1", post_id)
        .execute(&app_state.pool)
        .await?;
    return Ok(HttpResponse::Accepted().json(Response {
        message: "Pic deleted successfully".to_string(),
    }));
}

#[delete("/postimages/{image_id}")]
pub async fn delete_post_image(
    req: HttpRequest,
    path: web::Path<i32>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let image_id = path.into_inner();
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
    let img_owner_query = sqlx::query!(
        "SELECT user_id FROM posts WHERE id=(SELECT post_id FROM posts_images WHERE id=$1)",
        image_id
    )
    .fetch_one(&app_state.pool)
    .await?;
    let img_owner_id = img_owner_query.user_id;
    if img_owner_id != user_id {
        return Ok(HttpResponse::BadRequest().json(Response {
            message: "You cannot delete someone else's images".to_string(),
        }));
    }
    let delete_query = sqlx::query!(
        "DELETE FROM posts_images WHERE id=$1 RETURNING image_url",
        image_id
    )
    .fetch_one(&app_state.pool)
    .await?;
    let image_fname = delete_query.image_url;
    let del_result = tokio::fs::remove_file(format!("upload/{}", image_fname)).await;
    if del_result.is_err() {
        println!("File {} not found in HDD", image_fname);
    }
    return Ok(HttpResponse::Ok().json(Response {
        message: "Image deleted successfully".to_string(),
    }));
}
