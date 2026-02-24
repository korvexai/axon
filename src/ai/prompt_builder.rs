use serde::{Deserialize};
use serde_json::Value;

/// Format standard pentru tool call generat de AI
///
/// Modelul trebuie sÄƒ genereze ceva de genul:
///
/// ```json
/// {
///   "tool": "run_shell",
///   "args": {
///       "command": "cargo build"
///   }
/// }
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct ToolCall {
    pub tool: String,
    pub args: Value,
}

/// Detect JSON tool call inside AI output.
/// Returns Some(ToolCall) if valid JSON tool detected.
pub fn detect_tool_call(output: &str) -> Option<ToolCall> {

    // 1ï¸âƒ£ CautÄƒ primul bloc JSON Ã®n text
    let start = output.find('{')?;
    let end = output.rfind('}')?;

    if end <= start {
        return None;
    }

    let json_str = &output[start..=end];

    // 2ï¸âƒ£ ÃŽncearcÄƒ sÄƒ parseze JSON
    let parsed: Value = serde_json::from_str(json_str).ok()?;

    // 3ï¸âƒ£ VerificÄƒ dacÄƒ are structura tool call
    if parsed.get("tool").is_some() && parsed.get("args").is_some() {
        serde_json::from_value(parsed).ok()
    } else {
        None
    }
}

// === AXON_COMPAT: PromptBuilder stub ===
/// Minimal compatibility PromptBuilder.
/// Replace with your real prompt composition logic.
#[derive(Debug, Clone, Default)]
pub struct PromptBuilder {
    buf: String,
}

impl PromptBuilder {
    pub fn new() -> Self { Self { buf: String::new() } }

    pub fn push_line(mut self, s: impl AsRef<str>) -> Self {
        self.buf.push_str(s.as_ref());
        self.buf.push('\n');
        self
    }

    pub fn build(self) -> String { self.buf }
}

