use dotenvy::var;
use sqlx::postgres::{PgPool, PgPoolOptions};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

fn get_database_url() -> String {
    let db_host = var("DB_HOST").unwrap();
    let db_port = var("DB_PORT").unwrap();
    let db_name = var("DB_NAME").unwrap();
    let db_username = var("DB_USERNAME").unwrap();
    let db_password = var("DB_PASSWORD").unwrap();
    let database_url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        db_username, db_password, db_host, db_port, db_name
    );
    return database_url;
}

pub async fn get_appstate() -> AppState {
    let database_url: String = get_database_url();
    let pool: PgPool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await
        .unwrap();
    let app_state = AppState { pool };
    return app_state;
}
