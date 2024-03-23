mod routes;
mod types;
use crate::routes::create_contact::create_contact;
use crate::routes::index::index;
use axum::{routing::get, routing::post, Router};
use dotenv::dotenv;
use sqlx::sqlite::SqlitePool;
use std::env;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    let app = Router::new()
        .route("/", get(index))
        .route("/contact", post(create_contact))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
