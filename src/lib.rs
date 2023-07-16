mod routes;
mod models;
mod misc;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

use misc::appstate::get_appstate;

pub struct TNCarsApp;

impl TNCarsApp {
    pub fn new() -> Self {
        return TNCarsApp {};
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let port_string: String = dotenvy::var("PORT").expect("PORT must be set in .env");
        let port: u16 = port_string
            .parse::<u16>()
            .expect("PORT in .env must be a valid u16");
        let app_state = get_appstate().await;
        std::env::set_var("RUST_LOG", "actix_web=info");
        env_logger::init();

        println!("REST Backend running at https://localhost:{}", port);

        return HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .app_data(web::Data::new(app_state.clone()))
                .service(routes::hello::hello)
                .service(routes::users::create_user)
                .service(routes::users::login_user)
                .service(routes::users::get_current_user)
        })
        .bind(("127.0.0.1", port))?
        .run()
        .await;
    }
}

