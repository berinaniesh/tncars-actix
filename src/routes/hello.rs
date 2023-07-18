use crate::models::hello::HelloMessage;
use actix_web::{get, HttpResponse};

#[get("/")]
pub async fn hello() -> HttpResponse {
    let ans = HelloMessage {
        ..Default::default()
    };
    return HttpResponse::Ok().json(ans);
}
