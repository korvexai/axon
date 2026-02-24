 // src/telegram/bot.rs or src/util/logging.rs

pub async fn dispatch_telegram_message(bot: &Bot, chat_id: i64, text: &str) -> Result<(), MyError> {
    let mut attempts = 0;
    let max_attempts = 3;

    while attempts < max_attempts {
        match bot.send_message(chat_id, text).await {
            Ok(_) => {
                log::info!(target: "telegram/api", "SUCCESS: Message delivered to chat_id: {}", chat_id);
                return Ok(());
            }
            Err(e) => {
                attempts += 1;
                
                // We analyze the error type for "Granular Observability"
                match e {
                    // If it's a rate limit, we must wait (Backoff)
                    EroareTelegram::RateLimited(seconds) => {
                        log::warn!(target: "telegram/api", "RATE_LIMIT: Waiting {} seconds", seconds);
                        tokio::time::sleep(Duration::from_secs(seconds)).await;
                    },
                    // Network error – retry after a short delay
                    EroareTelegram::NetworkError => {
                        log::error!(target: "telegram/api", "RETRY {}: Network error for chat_id: {}", attempts, chat_id);
                        tokio::time::sleep(Duration::from_millis(500 * attempts)).await;
                    },
                    // Fatal error (e.g., ChatNotFound) – do not retry
                    _ => {
                        log::error!(target: "telegram/api", "FATAL: Cannot send message: {:?}", e);
                        return Err(e.into());
                    }
                }
            }
        }
    }
    Err(MyError::MaxRetriesExceeded)
}