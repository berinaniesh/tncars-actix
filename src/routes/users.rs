use crate::misc::appstate::AppState;
use crate::models::users::CreateUser;
use crate::models::Message;
use actix_web::{post, web, HttpResponse};
use crate::misc::hasher::hash;

#[post("/users")]
pub async fn create_user(
    app_state: web::Data<AppState>,
    form: web::Json<CreateUser>,
) -> HttpResponse {
    let hashed_pw = hash(&form.password);
    let created = sqlx::query!(
        "INSERT INTO users (email, password) values ($1, $2)",
        &form.email,
        &hashed_pw
    )
    .execute(&app_state.pool)
    .await;

    return HttpResponse::Ok().json(Message {
        message: "ok".to_string(),
    });
}
