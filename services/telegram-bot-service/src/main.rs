use telegram_bot_service::config::init_tracing;

#[tokio::main]
async fn main() {
    init_tracing();
}
