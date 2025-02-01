mod handlers;
mod state;

use axum::routing::{delete, get, post, put, Router};
use dotenv::dotenv;
use fred::prelude::*;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let postgres_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL env");
    let redis_url = std::env::var("REDIS_URL").expect("Missing REDIS_URL env");

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let postgres_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&postgres_url)
        .await?;

    let pool_size = 10;
    let redis_config = RedisConfig::from_url(&redis_url)?;

    let redis_pool = Builder::from_config(redis_config)
        .with_performance_config(|config| {
            config.auto_pipeline = true;
        })
        .set_policy(ReconnectPolicy::new_exponential(0, 100, 30_000, 2))
        .build_pool(pool_size)
        .expect("Failed to create redis pool");

    if std::env::var("REDIS_URL")? != "" {
        redis_pool.init().await.expect("Failed to connect to Redis");
        let _ = redis_pool.flushall::<i32>(false).await;
    }

    let state = Arc::new(Mutex::new(state::StateInternal::new(
        postgres_pool,
        redis_pool,
    )));

    let app = Router::new()
        .route("/", get(handlers::health))
        .route("/quotes", post(handlers::create_quote))
        .route("/quotes", get(handlers::read_all_quotes))
        .route("/quotes/{id}", get(handlers::read_one_quotes))
        .route("/quotes/{id}", put(handlers::update_one_quotes))
        .route("/quotes/{id}", delete(handlers::delete_one_quotes))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
