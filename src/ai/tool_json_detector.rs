use crate::event::event::{AxonEvent, WorkerHealth};
pub use crate::ai::model_router::ToolCall;

pub struct ToolRouter;

pub fn detect_tool_call(_text: &str) -> Option<ToolCall> {
    None
}

pub async fn handle_potential_tool_call(_text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let _ = AxonEvent::WorkerStatus { 
        name: "tool_detector".to_string(), 
        health: WorkerHealth::Healthy 
    };
    Ok(())
}
