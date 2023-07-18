use tncars_actix::TNCarsApp;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = TNCarsApp::new();
    return app.run().await;
}
