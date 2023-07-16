use serde::{Serialize, Deserialize};
use actix_web::{get, HttpResponse};

#[derive(Serialize, Deserialize, Debug)]
struct HelloMessage {
	greeting: String,
	about: String,
	authentication: String,
	framework: String,
	database: String,
}

impl Default for HelloMessage {
	fn default() -> HelloMessage {
		HelloMessage{
			greeting: String::from("Hello there!"),
			about: String::from("Backend for TNCars"),
			authentication: String::from("JWT"),
			framework: String::from("Actix-Web"),
			database: String::from("Postgresql"),
		}
	}
}

#[get("/")]
pub async fn hello() -> HttpResponse {
	let ans = HelloMessage {
		..Default::default()
	};
	return HttpResponse::Ok().json(ans);
}
