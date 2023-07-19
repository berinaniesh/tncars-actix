use actix_web::{post, web, HttpRequest, HttpResponse};
use crate::models::{Response, posts::CreatePost};
use crate::misc::appstate::AppState;
use crate::misc::jwt::get_id_from_request;

#[post("/posts")]
pub async fn create_post(req: HttpRequest, app_state: web::Data<AppState>, form: web::Json<CreatePost>) -> HttpResponse {
    let user_id_result = get_id_from_request(&req);
    if user_id_result.is_err() {
        return HttpResponse::BadRequest().json(Response {
            message: "Invalid authorization headers".to_string(),
        });
    }
    let user_id = user_id_result.unwrap();
    
    return HttpResponse::Ok().json(Response{message: form.title.clone()});
}