use crate::handlers::Quote;
use crate::state::AppState;
use std::error::Error;

static QUERY: &str = "
SELECT * FROM quotes
";

pub async fn read(state: AppState) -> Result<Vec<Quote>, Box<dyn Error>> {
    let postgres_db = &state.lock().await.postgres_database;

    let quote = sqlx::query_as(QUERY).fetch_all(postgres_db).await?;

    Ok(quote)
}
