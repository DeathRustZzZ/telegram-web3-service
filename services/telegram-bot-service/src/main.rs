use telegram_bot_service::config::{AppConfig, init_tracing};

#[tokio::main]
async fn main() {
    init_tracing();
    tracing::info!("Starting Telegram Bot Service...");
    let config = AppConfig::load_from_environment();
    tracing::debug!("Config loaded successfully");
}
