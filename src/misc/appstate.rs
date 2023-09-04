use sqlx::postgres::{PgPool, PgPoolOptions};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub async fn get_appstate() -> AppState {
    let database_url: String = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: PgPool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await
        .unwrap();
    let app_state = AppState { pool };
    return app_state;
}
