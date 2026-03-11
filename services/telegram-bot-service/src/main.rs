use telegram_bot_service::config::{AppConfig, init_tracing};
use telegram_bot_service::db::connection::init_pool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Читаем переменные окружения и создаём конфигурацию.
    dotenvy::dotenv().ok();
    init_tracing();
    tracing::info!("Starting Telegram Bot Service...");
    let config = AppConfig::load_from_environment()?;
    tracing::debug!("Config loaded successfully");

    tracing::info!("Initializing database connection pool...");
    let _pool = init_pool(&config.database_url).await;
    tracing::debug!("Database connection pool initialized successfully");

    tracing::info!("Initializing Telegram bot...");
    let _bot = teloxide::Bot::new(config.bot_token.clone());
    tracing::debug!("Telegram bot initialized successfully");

    tracing::warn!("Bot runtime stopped, application is shutting down");
    Ok(())
}
