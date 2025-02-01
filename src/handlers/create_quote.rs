use crate::handlers::Quote;
use crate::state::AppState;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct CreateQuote {
    book: String,
    quote: String,
    remarks: Option<String>,
}

static QUERY: &str = "
INSERT INTO quotes (uuid, book, quote, remarks, inserted_at, updated_at)
VALUES ($1, $2, $3, $4, $5, $6)
RETURNING *
";

pub async fn create(state: AppState, quote: CreateQuote) -> Result<Quote, Box<dyn Error>> {
    let postgres_db = &state.lock().await.postgres_database;

    let now = chrono::Utc::now();

    let quote = sqlx::query_as(QUERY)
        .bind(uuid::Uuid::new_v4())
        .bind(&quote.book)
        .bind(&quote.quote)
        .bind(&quote.remarks)
        .bind(now)
        .bind(now)
        .fetch_one(postgres_db)
        .await?;

    Ok(quote)
}
