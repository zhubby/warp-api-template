use sqlx::postgres::{PgPool,PgPoolOptions};
use crate::Args;

#[derive(Clone, Debug)]
pub struct Config {
    db_pool: PgPool,
}

impl Config {
    pub async fn new(args: &Args) -> anyhow::Result<Self> {
        let Args {
            database_url,
            ..
        } = &args;

        let db_pool = PgPoolOptions::new().max_connections(5).connect(database_url).await?;
        Ok(Self{db_pool})
    }

    pub fn db(&self) -> &PgPool{
        &self.db_pool
    }
}