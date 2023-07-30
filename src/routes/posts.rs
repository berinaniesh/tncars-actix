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
) -> HttpResponse {
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
    let query = sqlx::query_as!(PostOut,
        r#"
        SELECT id, title, user_id, brand, post_pic, price, model_year, km_driven, transmission as "transmission: _", fuel as "fuel: _", description, location, is_sold, created_at, updated_at FROM posts where id=$1
        "#, post_id) 
        .fetch_one(&app_state.pool)
        .await;

    if query.is_ok() {
        return HttpResponse::Ok().json(query.unwrap());
    }
    return HttpResponse::NotFound().json(Response {
        message: format!("Post with id: {} was not found", post_id),
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
    let q1 = sqlx::query_as!(UpdatedPost,
        r#"
        SELECT
        title, user_id, brand, price, model_year, km_driven, transmission as "transmission: _", fuel as "fuel: _", description, location, is_sold
        FROM posts WHERE id=$1
        "#, post_id)
        .fetch_one(&app_state.pool).await;

    if q1.is_err() {
        return HttpResponse::NotFound().json(Response {
            message: format!("Post with id: {} was not found in the database", post_id),
        });
    }
    let db_data = q1.unwrap();
    let user_id_db = db_data.user_id;
    let user_id_jwt: i32 = user_id;
    if user_id_db != user_id_jwt {
        return HttpResponse::Unauthorized().json(Response {
            message: "You cannot modify someone else's post".to_string(),
        });
    }
    let updated_post = get_updated_post(form, db_data);
    let q2 = sqlx::query_as!(PostOut,
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
        .await;
    if q2.is_err() {
        return HttpResponse::InternalServerError().json(Response {
            message: "Something went wrong, try again later".to_string(),
        });
    }
    let postout = q2.unwrap();
    return HttpResponse::Ok().json(postout);
}

#[delete("/posts/{id}")]
pub async fn delete_post(
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

    let q1 = sqlx::query!(
        r#"
        SELECT user_id FROM posts WHERE id=$1
        "#,
        post_id
    )
    .fetch_one(&app_state.pool)
    .await;

    if q1.is_err() {
        return HttpResponse::NotFound().json(Response {
            message: format!("Post with id: {} was not found in the database", post_id),
        });
    }
    let req_user = user_id;
    let db_user = q1.unwrap().user_id;
    if req_user != db_user {
        return HttpResponse::Unauthorized().json(Response {
            message: "You cannot delete someone else's post".to_string(),
        });
    }
    let delete_req = sqlx::query!("DELETE FROM posts WHERE id=$1", post_id)
        .execute(&app_state.pool)
        .await;
    if delete_req.is_err() {
        return HttpResponse::InternalServerError().json(Response {
            message: "Something went wrong".to_string(),
        });
    }
    return HttpResponse::Ok().json(Response {
        message: "Post deleted successfully".to_string(),
    });
}

#[get("/posts/{post_id}/comments")]
pub async fn get_comments(path: web::Path<i32>, app_state: web::Data<AppState>) -> HttpResponse {
    let post_id = path.into_inner();
    let q1 = sqlx::query_as!(
        CommentOut,
        "SELECT * FROM comments WHERE post_id=$1",
        post_id
    )
    .fetch_all(&app_state.pool)
    .await;
    if q1.is_ok() {
        return HttpResponse::Ok().json(q1.unwrap());
    }
    return HttpResponse::InternalServerError().json(Response {
        message: "Something went wrong, try again later".to_string(),
    });
}

#[get("/posts/{post_id}/images")]
pub async fn get_post_images(path: web::Path<i32>, app_state: web::Data<AppState>) -> HttpResponse {
    let post_id = path.into_inner();
    let q1 = sqlx::query_as!(
        ImagesOut,
        "SELECT id, image_url, created_at FROM posts_images WHERE post_id=$1",
        post_id
    )
    .fetch_all(&app_state.pool)
    .await;
    if q1.is_err() {
        return HttpResponse::InternalServerError().json(Response {
            message: "Something went wrong, try again later".to_string(),
        });
    }
    return HttpResponse::Ok().json(q1.unwrap());
}

#[delete("/posts/{post_id}/postpic")]
pub async fn delete_post_primary_image(
    req: HttpRequest,
    path: web::Path<i32>,
    app_state: web::Data<AppState>,
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
    let user_id_post_result = sqlx::query_as!(
        PostImg,
        "SELECT user_id, post_pic FROM posts WHERE id=$1",
        post_id
    )
    .fetch_one(&app_state.pool)
    .await;
    if user_id_post_result.is_err() {
        return HttpResponse::NotFound().json(Response {
            message: "Post not found".to_string(),
        });
    }
    let post_img = user_id_post_result.unwrap();
    if user_id != post_img.user_id {
        return HttpResponse::Unauthorized().json(Response {
            message: "You cannot delete someone else's posts's pic".to_string(),
        });
    }
    let post_pic_opt = post_img.post_pic;
    if post_pic_opt.is_none() {
        return HttpResponse::NotAcceptable().json(Response {
            message: "The post does not have a primary pic".to_string(),
        });
    }
    let post_pic_fname = post_pic_opt.unwrap();
    let del_result = tokio::fs::remove_file(format!("upload/{}", post_pic_fname)).await;
    if del_result.is_err() {
        println!("File {} not found in HDD", post_pic_fname);
    }
    let set_null_query = sqlx::query!("UPDATE posts SET post_pic=NULL WHERE id=$1", post_id)
        .execute(&app_state.pool)
        .await;
    if set_null_query.is_err() {
        return HttpResponse::InternalServerError().json(Response {
            message: "Something went wrong, try again later".to_string(),
        });
    }
    return HttpResponse::Accepted().json(Response {
        message: "Pic deleted successfully".to_string(),
    });
}

#[delete("/postimages/{image_id}")]
pub async fn delete_post_image(
    req: HttpRequest,
    path: web::Path<i32>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let image_id = path.into_inner();
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
    let img_owner_query = sqlx::query!(
        "SELECT user_id FROM posts WHERE id=(SELECT post_id FROM posts_images WHERE id=$1)",
        image_id
    )
    .fetch_one(&app_state.pool)
    .await;
    if img_owner_query.is_err() {
        return HttpResponse::NotFound().json(Response {
            message: "Requested image not found".to_string(),
        });
    }
    let img_owner_id = img_owner_query.unwrap().user_id;
    if img_owner_id != user_id {
        return HttpResponse::BadRequest().json(Response {
            message: "You cannot delete someone else's images".to_string(),
        });
    }
    let delete_query = sqlx::query!(
        "DELETE FROM posts_images WHERE id=$1 RETURNING image_url",
        image_id
    )
    .fetch_one(&app_state.pool)
    .await;
    if delete_query.is_err() {
        return HttpResponse::InternalServerError().json(Response {
            message: "Something went wrong, try again later".to_string(),
        });
    }
    let image_fname = delete_query.unwrap().image_url;
    let del_result = tokio::fs::remove_file(format!("upload/{}", image_fname)).await;
    if del_result.is_err() {
        println!("File {} not found in HDD", image_fname);
    }
    return HttpResponse::Ok().json(Response {
        message: "Image deleted successfully".to_string(),
    });
}
