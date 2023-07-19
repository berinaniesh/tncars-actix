use crate::misc::appstate::AppState;
use crate::misc::jwt::get_id_from_request;
use crate::models::{posts::CreatePost, Response};
use actix_web::{post, web, HttpRequest, HttpResponse};

#[post("/posts")]
pub async fn create_post(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    fixed_form: web::Json<CreatePost>,
) -> HttpResponse {
    let user_id_result = get_id_from_request(&req);
    if user_id_result.is_err() {
        return HttpResponse::BadRequest().json(Response {
            message: "Invalid authorization headers".to_string(),
        });
    }
    let user_id = user_id_result.unwrap();
    //let fixed_form = fix_createpost_form(&form);
    let query = sqlx::query(
        "INSERT INTO posts (title, user_id, brand, price, model_year, km_driven, transmission, fuel, description, location) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)") 
        .bind(&fixed_form.title)
        .bind(user_id) 
        .bind(&fixed_form.brand)
        .bind(&fixed_form.price) 
        .bind(&fixed_form.model_year)
        .bind(&fixed_form.km_driven) 
        .bind(&fixed_form.transmission)
        .bind(&fixed_form.fuel)
        .bind(&fixed_form.description)
        .bind(&fixed_form.location)
        .execute(&app_state.pool)
        .await;
    if query.is_ok() {
        return HttpResponse::Created().json(Response {
            message: "Post created successfully".to_string(),
        });
    }
    dbg!(query);
    return HttpResponse::InternalServerError().json(Response {
        message: "Something went wrong, try again later".to_string(),
    });
}
