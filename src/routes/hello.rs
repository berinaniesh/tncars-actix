use actix_web::{get, HttpResponse};
use crate::models::hello::HelloMessage;

#[get("/")]
pub async fn hello() -> HttpResponse {
	let ans = HelloMessage {
		..Default::default()
	};
	return HttpResponse::Ok().json(ans);
}
