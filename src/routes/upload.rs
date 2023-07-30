use crate::misc::appstate::AppState;
use crate::misc::constants::MAX_FILE_SIZE;
use crate::misc::jwt::get_id_from_request;
use crate::models::Response;
use actix_multipart::Multipart;
use actix_web::{http::header::CONTENT_LENGTH, post, web, HttpRequest, HttpResponse};
use futures_util::TryStreamExt as _;
use image::{imageops::FilterType, DynamicImage};
use mime::{Mime, IMAGE_JPEG, IMAGE_PNG};
use tokio::fs;
use tokio::io::AsyncWriteExt as _;
use uuid::Uuid;

#[post("/upload/profilepic")]
pub async fn upload_profilepic(
    mut payload: Multipart,
    req: HttpRequest,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let content_length: usize = match req.headers().get(CONTENT_LENGTH) {
        Some(header_value) => header_value.to_str().unwrap_or("0").parse().unwrap_or(0),
        None => 0,
    };

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

    let legal_filetypes: [Mime; 2] = [IMAGE_PNG, IMAGE_JPEG];
    let dir: &str = "./upload/";

    if content_length > MAX_FILE_SIZE {
        return HttpResponse::BadRequest().json(Response {
            message: "File too big, max allowed size is 2 MB".to_string(),
        });
    }

    if let Ok(Some(mut field)) = payload.try_next().await {
        let filetype: Option<&Mime> = field.content_type();
        if filetype.is_none() || !legal_filetypes.contains(&filetype.unwrap()) {
            return HttpResponse::BadRequest().json(Response {
                message: "Only jpeg/png files allowed".to_string(),
            });
        }
        let destination: String = format!(
            "{}{}-{}",
            dir,
            Uuid::new_v4(),
            field.content_disposition().get_filename().unwrap()
        );

        let mut saved_file: fs::File = fs::File::create(&destination).await.unwrap();
        while let Ok(Some(chunk)) = field.try_next().await {
            let _ = saved_file.write_all(&chunk).await.unwrap();
        }
        let new_fname = Uuid::new_v4().to_string() + ".jpeg";
        let new_fname2 = new_fname.clone(); // needed for closure move
        web::block(move || async move {
            let uploaded_img: DynamicImage = image::open(&destination).unwrap();
            let _ = fs::remove_file(&destination).await.unwrap();
            uploaded_img
                .resize(1024, 768, FilterType::Lanczos3)
                .save(format!("{}{}", dir, new_fname2))
                .unwrap();
        })
        .await
        .unwrap()
        .await;
        let q1 = sqlx::query!("SELECT profile_pic FROM users WHERE id=$1", user_id)
            .fetch_one(&app_state.pool)
            .await;
        if q1.is_ok() {
            let q1_img = q1.unwrap().profile_pic;
            if q1_img.is_some() {
                let old_image_fname = q1_img.unwrap();
                let del_result = fs::remove_file(format!("upload/{}", old_image_fname)).await;
                if del_result.is_err() {
                    println!("File {} not found in HDD", old_image_fname);
                }
            }
        }
        let _ = sqlx::query!(
            "UPDATE users SET profile_pic=$1 WHERE id=$2",
            &new_fname,
            user_id
        )
        .execute(&app_state.pool)
        .await
        .unwrap();
    }

    return HttpResponse::Ok().json(Response {
        message: "Profile Pic updated successfully".to_string(),
    });
}

#[post("/upload/postprimarypic/{id}")]
pub async fn upload_post_primary_pic(
    path: web::Path<i32>,
    mut payload: Multipart,
    req: HttpRequest,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let post_id: i32 = path.into_inner();
    let content_length: usize = match req.headers().get(CONTENT_LENGTH) {
        Some(header_value) => header_value.to_str().unwrap_or("0").parse().unwrap_or(0),
        None => 0,
    };

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

    let q0 = sqlx::query!("SELECT user_id FROM posts WHERE id=$1", post_id)
        .fetch_one(&app_state.pool)
        .await;
    if q0.is_err() {
        return HttpResponse::NotFound().json(Response {
            message: format!("Post with id {} not found", post_id),
        });
    }

    let post_user_id = q0.unwrap().user_id;

    if post_user_id != user_id {
        return HttpResponse::Unauthorized().json(Response {
            message: "You cannot change the pics of someone else's post".to_string(),
        });
    }

    let legal_filetypes: [Mime; 2] = [IMAGE_PNG, IMAGE_JPEG];
    let dir: &str = "./upload/";

    if content_length > MAX_FILE_SIZE {
        return HttpResponse::BadRequest().json(Response {
            message: "File too big, max allowed size is 2 MB".to_string(),
        });
    }

    if let Ok(Some(mut field)) = payload.try_next().await {
        let filetype: Option<&Mime> = field.content_type();
        if filetype.is_none() || !legal_filetypes.contains(&filetype.unwrap()) {
            return HttpResponse::BadRequest().json(Response {
                message: "Only jpeg/png files allowed".to_string(),
            });
        }
        let destination: String = format!(
            "{}{}-{}",
            dir,
            Uuid::new_v4(),
            field.content_disposition().get_filename().unwrap()
        );

        let mut saved_file: fs::File = fs::File::create(&destination).await.unwrap();
        while let Ok(Some(chunk)) = field.try_next().await {
            let _ = saved_file.write_all(&chunk).await.unwrap();
        }
        let new_fname = Uuid::new_v4().to_string() + ".jpeg";
        let new_fname2 = new_fname.clone(); // needed for closure move
        web::block(move || async move {
            let uploaded_img: DynamicImage = image::open(&destination).unwrap();
            let _ = fs::remove_file(&destination).await.unwrap();
            uploaded_img
                .resize(1024, 768, FilterType::Lanczos3)
                .save(format!("{}{}", dir, new_fname2))
                .unwrap();
        })
        .await
        .unwrap()
        .await;
        let q1 = sqlx::query!("SELECT post_pic FROM posts WHERE id=$1", post_id)
            .fetch_one(&app_state.pool)
            .await;
        if q1.is_ok() {
            let q1_img = q1.unwrap().post_pic;
            if q1_img.is_some() {
                let old_image_fname = q1_img.unwrap();
                let del_result = fs::remove_file(format!("upload/{}", old_image_fname)).await;
                if del_result.is_err() {
                    println!("File {} not found in HDD", old_image_fname);
                }
            }
        }
        let _ = sqlx::query!(
            "UPDATE posts SET post_pic=$1 WHERE id=$2",
            &new_fname,
            post_id
        )
        .execute(&app_state.pool)
        .await
        .unwrap();
    }

    return HttpResponse::Ok().json(Response {
        message: "Post primary pic updated successfully".to_string(),
    });
}

#[post("/upload/postpics/{id}")]
pub async fn upload_post_pic(
    path: web::Path<i32>,
    mut payload: Multipart,
    req: HttpRequest,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let post_id: i32 = path.into_inner();
    let content_length: usize = match req.headers().get(CONTENT_LENGTH) {
        Some(header_value) => header_value.to_str().unwrap_or("0").parse().unwrap_or(0),
        None => 0,
    };

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

    let q0 = sqlx::query!("SELECT user_id FROM posts WHERE id=$1", post_id)
        .fetch_one(&app_state.pool)
        .await;
    if q0.is_err() {
        return HttpResponse::NotFound().json(Response {
            message: format!("Post with id {} not found", post_id),
        });
    }

    let post_user_id = q0.unwrap().user_id;

    if post_user_id != user_id {
        return HttpResponse::Unauthorized().json(Response {
            message: "You cannot change the pics of someone else's post".to_string(),
        });
    }

    let count_query = sqlx::query!(
        "SELECT COUNT(id) FROM posts_images WHERE post_id=$1",
        post_id
    )
    .fetch_one(&app_state.pool)
    .await;
    let pic_count = count_query.unwrap().count.unwrap();
    if pic_count >= 10 {
        return HttpResponse::NotAcceptable().json(Response {
            message: "Only a max of 10 pictures per post allowed".to_string(),
        });
    }

    let legal_filetypes: [Mime; 2] = [IMAGE_PNG, IMAGE_JPEG];
    let dir: &str = "./upload/";

    if content_length > MAX_FILE_SIZE {
        return HttpResponse::BadRequest().json(Response {
            message: "File too big, max allowed size is 2 MB".to_string(),
        });
    }

    if let Ok(Some(mut field)) = payload.try_next().await {
        let filetype: Option<&Mime> = field.content_type();
        if filetype.is_none() || !legal_filetypes.contains(&filetype.unwrap()) {
            return HttpResponse::BadRequest().json(Response {
                message: "Only jpeg/png files allowed".to_string(),
            });
        }
        let destination: String = format!(
            "{}{}-{}",
            dir,
            Uuid::new_v4(),
            field.content_disposition().get_filename().unwrap()
        );

        let mut saved_file: fs::File = fs::File::create(&destination).await.unwrap();
        while let Ok(Some(chunk)) = field.try_next().await {
            let _ = saved_file.write_all(&chunk).await.unwrap();
        }
        let new_fname = Uuid::new_v4().to_string() + ".jpeg";
        let new_fname2 = new_fname.clone(); // needed for closure move
        web::block(move || async move {
            let uploaded_img: DynamicImage = image::open(&destination).unwrap();
            let _ = fs::remove_file(&destination).await.unwrap();
            uploaded_img
                .resize(1024, 768, FilterType::Lanczos3)
                .save(format!("{}{}", dir, new_fname2))
                .unwrap();
        })
        .await
        .unwrap()
        .await;

        let insert_query = sqlx::query!(
            "INSERT INTO posts_images (post_id, image_url) VALUES ($1, $2)",
            post_id,
            &new_fname
        )
        .execute(&app_state.pool)
        .await;
        if insert_query.is_ok() {
            return HttpResponse::Created().json(Response {
                message: "Pic added successfully".to_string(),
            });
        }
    }

    return HttpResponse::InternalServerError().json(Response {
        message: "Something went wrong, try again later".to_string(),
    });
}
