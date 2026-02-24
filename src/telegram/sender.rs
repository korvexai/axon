use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::ParseMode;

pub async fn send(
    token: &str,
    chat_id: i64,
    text: &str,
) -> Result<()> {
    let bot = Bot::new(token);

    bot.send_message(ChatId(chat_id), text)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;

    Ok(())
}
