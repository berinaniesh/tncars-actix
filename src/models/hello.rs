use serde::Serialize;

#[derive(Serialize)]
pub struct HelloMessage {
	pub greeting: String,
	pub about: String,
	pub authentication: String,
	pub framework: String,
	pub database: String,
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