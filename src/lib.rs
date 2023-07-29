mod misc;
mod models;
mod routes;

use actix_files as fs;
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
                .service(fs::Files::new("/static", "./upload").show_files_listing())
                .service(routes::hello::hello)
                .service(routes::users::create_user)
                .service(routes::users::login_user)
                .service(routes::users::refresh_token)
                .service(routes::users::get_current_user)
                .service(routes::users::get_email_otp)
                .service(routes::verify::email_otp)
                .service(routes::verify::email_url)
                .service(routes::users::update_user)
                .service(routes::posts::create_post)
                .service(routes::posts::get_post)
                .service(routes::posts::update_post)
                .service(routes::posts::delete_post)
                .service(routes::users::delete_user)
                .service(routes::likes::add_like)
                .service(routes::comments::add_comment)
                .service(routes::comments::get_specific_comment)
                .service(routes::comments::delete_comment)
                .service(routes::comments::change_comment)
                .service(routes::comments::get_comments)
                .service(routes::utils::is_username_available)
                .service(routes::users::get_user)
                .service(routes::users::get_users_posts)
                .service(routes::follows::follow_user)
                .service(routes::follows::get_following)
                .service(routes::follows::get_followed_by)
                .service(routes::upload::upload_profilepic)
                .service(routes::users::forgot_password)
                .service(routes::users::change_password)
        })
        .bind(("127.0.0.1", port))?
        .run()
        .await;
    }
}
