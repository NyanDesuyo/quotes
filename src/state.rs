#![allow(dead_code)]
use crate::handlers::Quote;
use fred::interfaces::KeysInterface;
use fred::{clients::RedisPool, prelude::*};
use serde_json::Value;
use sqlx::postgres::PgPool;
use std::error::Error;

pub struct StateInternal {
    pub postgres_database: sqlx::postgres::PgPool,
    pub redis_database: Cache,
}

impl StateInternal {
    pub fn new(db: PgPool, redis: RedisPool) -> Self {
        StateInternal {
            postgres_database: db,
            redis_database: Cache { internal: redis },
        }
    }
}

pub struct Cache {
    internal: RedisPool,
}

impl Cache {
    fn key_for_id(id: uuid::Uuid) -> String {
        format!("quote:{}", id)
    }

    pub async fn get(&mut self, id: uuid::Uuid) -> Result<Option<Quote>, Box<dyn Error>> {
        if !self.internal.is_connected() {
            return Err(Box::new(simple_error::SimpleError::new(
                "not connected redis",
            )));
        }

        let value: Option<Value> = self.internal.get(Self::key_for_id(id)).await?;

        let quote = match value {
            Some(x) => match serde_json::from_value(x) {
                Ok(x) => Some(x),
                Err(_) => None,
            },
            None => None,
        };
        Ok(quote)
    }

    pub async fn set(
        &mut self,
        id: uuid::Uuid,
        quote: &Quote,
        expiration: Option<Expiration>,
        set_opts: Option<SetOptions>,
        get: bool,
    ) -> Result<(), Box<dyn Error>> {
        if !self.internal.is_connected() {
            return Err(Box::new(simple_error::SimpleError::new(
                "not connected redis",
            )));
        }

        let value: Value = serde_json::to_value(quote)?;
        let key = Self::key_for_id(id);
        self.internal
            .set(key, value.to_string(), expiration, set_opts, get)
            .await?;
        Ok(())
    }

    pub async fn del(&mut self, id: uuid::Uuid) -> Result<(), Box<dyn Error>> {
        let key = Self::key_for_id(id);
        self.internal.del(key).await?;
        Ok(())
    }
}

pub type AppState = std::sync::Arc<tokio::sync::Mutex<StateInternal>>;
