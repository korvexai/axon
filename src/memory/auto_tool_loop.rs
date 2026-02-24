use uuid::Uuid;
use serde_json::Value;

pub async fn tool_reasoning_loop<F>(
    _conversation_id: Uuid,
    mut response: String,
    mut executor: F,
) where
    F: FnMut(String, Value) -> String,
{
    for _ in 0..3 {
        if let Some((tool, args)) = crate::memory::tool_schema_detector::detect_tool_call(&response) {
            response = executor(tool, args);
        } else {
            break;
        }
    }
}






