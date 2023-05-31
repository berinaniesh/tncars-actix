mod app;
mod middleware;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

use middleware::appstate::get_appstate;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
            .service(app::hello::hello)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await;
}
