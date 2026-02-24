use uuid::Uuid;

pub fn format_approval(
    request_id: Uuid,
    description: Option<String>,
) -> String {
    let desc = description.unwrap_or_default();

    format!(
        "âš ï¸ *Approval Required*\n\n*ID:* `{}`\n\n{}",
        request_id,
        desc
    )
}
