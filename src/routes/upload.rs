use tokio::fs;
use tokio::io::AsyncWriteExt as _;
use actix_web::{ HttpResponse,
                 HttpRequest,
                 web,
                 post,
                 http::header::CONTENT_LENGTH };
use actix_multipart::{ Multipart };
use futures_util::{ TryStreamExt as _ };
use mime::{ Mime, IMAGE_PNG, IMAGE_JPEG };
use uuid::Uuid;
use crate::misc::jwt::get_id_from_request;
use crate::models::Response;
use crate::misc::appstate::AppState;
use image::{ DynamicImage, imageops::FilterType };

#[post("/upload/profilepic")]
pub async fn upload_profilepic(mut payload: Multipart, req: HttpRequest, app_state: web::Data<AppState>) -> HttpResponse {
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

    let max_file_size: usize = 1_048_576; // 1 MB (in bytes)
    let legal_filetypes: [Mime; 2] = [IMAGE_PNG, IMAGE_JPEG];
    let dir: &str = "./upload/";

    if content_length > max_file_size { 
        return HttpResponse::BadRequest().json( Response {
            message: "File too big, max allowed size is 1 MB".to_string()
        });
    }

    if let Ok(Some(mut field)) = payload.try_next().await {
        let filetype: Option<&Mime> = field.content_type();
        if filetype.is_none() || !legal_filetypes.contains(&filetype.unwrap()) { 
            return HttpResponse::BadRequest().json(Response{message: "Only jpeg/png files allowed".to_string()}) 
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
                .resize(640, 480, FilterType::Lanczos3)
                .save(format!("{}{}", dir, new_fname2)).unwrap();
        }).await.unwrap().await;
        let _ = sqlx::query!("UPDATE users SET profile_pic=$1 WHERE id=$2", &new_fname, user_id).execute(&app_state.pool).await.unwrap();
    }

    return HttpResponse::Ok().json(Response{
        message: "Profile Pic updated successfully".to_string()
    });
}
