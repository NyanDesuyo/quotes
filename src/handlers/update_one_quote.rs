#![allow(unused_imports)]
use crate::handlers::Quote;
use crate::state::{self, AppState};
use fred::prelude::*;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct UpdateQuote {
    book: String,
    quote: String,
    remarks: Option<String>,
}

static QUERY: &str = "
UPDATE quotes
SET book = $1, quote = $2, remarks = $3 ,updated_at = $4
WHERE uuid = $5
RETURNING *
";

pub async fn update(
    state: AppState,
    id: uuid::Uuid,
    body: UpdateQuote,
) -> Result<Option<Quote>, Box<dyn Error>> {
    tracing::info!("Updating quotes: {}", id);

    let now = chrono::Utc::now();

    let s = state.lock().await;

    let res: Option<Quote> = sqlx::query_as(QUERY)
        .bind(body.book)
        .bind(body.quote)
        .bind(body.remarks)
        .bind(now)
        .bind(id)
        .fetch_optional(&s.postgres_database)
        .await?;

    if let Some(quote) = &res {
        tracing::info!("updating cache if exists!");

        let quote = quote.clone();
        let state = state.clone();

        tokio::spawn(async move {
            let mut s = state.lock().await;
            let _ = s
                .redis_database
                .set(
                    id,
                    &quote,
                    Some(Expiration::EX(60)),
                    Some(SetOptions::XX),
                    false,
                )
                .await;
        });
    }

    Ok(res)
}
