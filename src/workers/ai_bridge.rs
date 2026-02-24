use crate::ai::model_router;
use std::path::Path;

pub async fn route_and_query(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    model_router::route_and_query(prompt).await
}

pub async fn suggest_fix(code: &str, instruction: &str) -> Result<String, Box<dyn std::error::Error>> {
    let prompt = format!("Code:\n{}\n\nInstruction: {}", code, instruction);
    model_router::route_and_query(&prompt).await
}

pub async fn validate_fix(code: &str, fix: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let prompt = format!("Code: {}\nFix: {}\nIs this valid? Reply ONLY with VALID or INVALID", code, fix);
    let res = model_router::route_and_query(&prompt).await?;
    Ok(res.contains("VALID"))
}

pub async fn apply_ai_patch(path: &Path, _errors: String) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Applying patch to {:?}", path);
    Ok(())
}
