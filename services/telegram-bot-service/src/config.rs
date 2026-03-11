use std::env;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("Info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_thread_ids(true)
                .with_line_number(true),
        )
        .init();
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Токен Telegram-бота.
    ///
    /// Критически важный параметр:
    /// без него приложение не имеет смысла продолжать работу.
    pub bot_token: String,
    /// URL подключения к базе данных.
    ///
    /// Используется при инициализации пула соединений.
    /// Неверное значение приведёт к падению приложения на старте,
    /// что является корректным поведением для backend-сервиса.
    pub database_url: String,
}

impl AppConfig {
    pub fn load_from_environment() -> Self {
        // Читаем переменные окружения и создаём конфигурацию.
        dotenvy::dotenv().ok();
        tracing::debug!("Loading configuration from environment variables...");

        // Загрузка токена Telegram-бота.
        let bot_token =
            env::var("BOT_TOKEN").expect("BOT_TOKEN is not set in environment variables");
        tracing::debug!("Loaded BOT_TOKEN from environment variables.");

        // Загрузка токена базы данных
        let database_url =
            env::var("DATABASE_URL").expect("DATABASE_URL is not set in environment variables");
        tracing::debug!("Loaded DATABASE_URL from environment variables.");

        Self {
            bot_token,
            database_url,
        }
    }
}
