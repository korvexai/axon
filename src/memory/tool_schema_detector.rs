use serde_json::Value;

pub fn detect_tool_call(response: &str) -> Option<(String, Value)> {
    if let Ok(json) = serde_json::from_str::<Value>(response) {
        if let Some(tool) = json.get("tool") {
            let name = tool.as_str()?.to_string();
            let args = json.get("args").cloned().unwrap_or(Value::Null);
            return Some((name, args));
        }
    }
    None
}






