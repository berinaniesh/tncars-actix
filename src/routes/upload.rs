use crate::misc::appstate::AppState;
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

    let max_file_size: usize = 2_097_152; // 2 MB (in bytes)
    let legal_filetypes: [Mime; 2] = [IMAGE_PNG, IMAGE_JPEG];
    let dir: &str = "./upload/";

    if content_length > max_file_size {
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
        let q1 = sqlx::query!("SELECT profile_pic FROM users WHERE id=$1", user_id).fetch_one(&app_state.pool).await;
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
