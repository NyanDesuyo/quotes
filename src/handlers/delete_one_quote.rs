use crate::handlers::Quote;
use crate::state::AppState;
use std::error::Error;

static QUERY: &str = "
DELETE FROM quotes
WHERE uuid = $1
";

pub async fn delete(state: AppState, id: uuid::Uuid) -> Result<u64, Box<dyn Error>> {
    tracing::info!("Deleting quote: {}", id);

    let mut s = state.lock().await;

    let cached: Option<Quote> = s.redis_database.get(id).await.unwrap_or(None);

    if let Some(_quote) = cached {
        tracing::info!("Get cached version");

        let state = state.clone();

        tokio::spawn(async move {
            let mut s = state.lock().await;

            tracing::info!("Delete in cache");
            let _ = s.redis_database.del(id).await;
        });
    }

    let res = sqlx::query(QUERY)
        .bind(id)
        .execute(&s.postgres_database)
        .await?;

    Ok(res.rows_affected())
}
