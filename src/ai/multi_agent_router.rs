use serde::{Serialize, Deserialize};

/// Represents one atomic change in a patch plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchAction {
    pub file_path: String,
    pub description: String,
    pub change_type: PatchType,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatchType {
    CreateFile,
    ModifyFile,
    DeleteFile,
    ReplaceBlock,
}

/// Tree structure for complex patch planning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchNode {
    pub id: String,
    pub description: String,
    pub actions: Vec<PatchAction>,
    pub children: Vec<PatchNode>,
}

impl PatchNode {

    pub fn new(id: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            actions: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn add_action(&mut self, action: PatchAction) {
        self.actions.push(action);
    }

    pub fn add_child(&mut self, child: PatchNode) {
        self.children.push(child);
    }

    /// Flatten tree into executable action list
    pub fn flatten(&self) -> Vec<PatchAction> {
        let mut all = Vec::new();

        for action in &self.actions {
            all.push(action.clone());
        }

        for child in &self.children {
            all.extend(child.flatten());
        }

        all
    }
}

// === AXON_COMPAT: MultiAgentRouter stub ===
/// Compatibility stub for MultiAgentRouter.
#[derive(Debug, Clone)]
pub struct MultiAgentRouter;

impl MultiAgentRouter {
    pub fn new() -> Self { MultiAgentRouter }
}

