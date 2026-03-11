use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

/// Инициализация пула соединений PostgreSQL.
///
/// Это инфраструктурная точка старта приложения.
/// Если соединение с БД не удалось — приложение **не может**
/// продолжать работу в корректном состоянии.
///
/// Поэтому:
/// - при успехе логируем `info`
/// - при ошибке логируем `error` и завершаем процесс через `panic`
pub async fn init_pool(database_url: &str) -> PgPool {
    tracing::info!("Init pool connection");
    match PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
    {
        Ok(pool) => {
            // Успешное подключение к базе данных
            tracing::info!("Successful database connection");
            pool
        }

        Err(e) => {
            // Ошибка подключения к БД — критическая.
            //
            // Возможные причины:
            // - неверный DATABASE_URL
            // - БД недоступна (docker/container не запущен)
            // - проблемы с сетью / DNS
            // - превышен лимит подключений на стороне БД
            //
            // Логируем error, чтобы информация точно попала в логи
            // даже при уровне RUST_LOG=info.
            tracing::error!("Error database connection: {}", e);

            panic!("Faild to connect to the database");
        }
    }
}
