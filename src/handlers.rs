mod create_quote;
mod delete_one_quote;
mod read_all_quotes;
mod read_one_quote;
mod update_one_quote;

use crate::state::AppState;
use axum::http::StatusCode;
use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Quote {
    id: i64,
    uuid: uuid::Uuid,
    book: String,
    quote: String,
    remarks: Option<String>,
    inserted_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

pub async fn health() -> StatusCode {
    StatusCode::OK
}

pub async fn create_quote(
    State(state): State<AppState>,
    Json(body): Json<create_quote::CreateQuote>,
) -> Result<(StatusCode, Json<Quote>), StatusCode> {
    let res = create_quote::create(state, body).await;

    match res {
        Ok(x) => Ok((StatusCode::CREATED, Json(x))),
        Err(e) => {
            tracing::error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn read_all_quotes(
    State(state): State<AppState>,
) -> Result<Json<Vec<Quote>>, StatusCode> {
    let res = read_all_quotes::read(state).await;

    match res {
        Ok(x) => Ok(Json(x)),
        Err(e) => {
            tracing::error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn read_one_quotes(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>, //id<i64>,
) -> Result<Json<Quote>, StatusCode> {
    let res = read_one_quote::find_by_id(state, id).await;

    match res {
        Ok(x) => match x {
            Some(x) => Ok(Json(x)),
            None => Err(StatusCode::NOT_FOUND),
        },
        Err(e) => {
            tracing::error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_one_quotes(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(body): Json<update_one_quote::UpdateQuote>,
) -> StatusCode {
    let res = update_one_quote::update(state, id, body).await;

    match res {
        Ok(x) => match x {
            None => StatusCode::NOT_FOUND,
            _ => StatusCode::OK,
        },
        Err(e) => {
            tracing::error!("{}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn delete_one_quotes(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> StatusCode {
    let res = delete_one_quote::delete(state, id).await;

    match res {
        Ok(rows) => match rows {
            0 => StatusCode::NOT_FOUND,
            _ => StatusCode::OK,
        },
        Err(e) => {
            tracing::error!("{}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
