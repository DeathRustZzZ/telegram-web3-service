use anyhow::{Context, Result};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

/// Инициализация пула соединений PostgreSQL.
///
/// Это инфраструктурная точка старта приложения.
/// Если соединение с БД не удалось — приложение **не может**
/// продолжать работу в корректном состоянии.
pub async fn init_pool(database_url: &str) -> Result<PgPool> {
    tracing::info!("Init pool connection");
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .context("Faild to connect to the database...")
}
