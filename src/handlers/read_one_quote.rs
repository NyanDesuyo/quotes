#![allow(unused_imports)]
use crate::handlers::Quote;
use crate::state::AppState;
use fred::prelude::*;
use std::error::Error;

static QUERY: &str = "
SELECT * FROM quotes
WHERE uuid = $1
";

pub async fn find_by_id(state: AppState, id: uuid::Uuid) -> Result<Option<Quote>, Box<dyn Error>> {
    let mut s = state.lock().await;

    let cached: Option<Quote> = s.redis_database.get(id).await.unwrap_or(None);

    if let Some(quote) = cached {
        tracing::info!("Returning cached version");
        return Ok(Some(quote));
    }

    let res: Option<Quote> = sqlx::query_as(QUERY)
        .bind(id)
        .fetch_optional(&s.postgres_database)
        .await?;

    if let Some(quote) = &res {
        let quote = quote.clone();
        let state = state.clone();

        tokio::spawn(async move {
            let mut s = state.lock().await;

            tracing::info!("Storing in cache");
            let _ = s
                .redis_database
                .set(id, &quote, Some(Expiration::EX(60)), None, false)
                .await;
        });
    }

    tracing::info!("Returning database version");

    Ok(res)
}
