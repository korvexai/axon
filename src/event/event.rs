use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkerHealth {
    Running,
    Idle,
    Stopped,
    Healthy,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
}

impl LogLevel {
    pub fn is_actionable(&self) -> bool {
        matches!(self, LogLevel::Error | LogLevel::Warn)
    }

    pub fn from_line(line: &str) -> LogLevel {
        let l = line.to_uppercase();
        if l.contains("ERROR") { LogLevel::Error }
        else if l.contains("WARN") { LogLevel::Warn }
        else if l.contains("DEBUG") { LogLevel::Debug }
        else { LogLevel::Info }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRecord {
    pub id: Uuid,
    pub level: LogLevel,
    pub message: String,
    pub source: String,
    pub timestamp: u64,
    pub fix_applied: bool,
    pub resolved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AxonEvent {
    AiRequest { 
        id: Uuid, 
        prompt: String, 
        model: Option<String>, 
        context: Option<String>,
    },
    AiResponse { 
        request_id: Uuid, 
        output: String, 
        model: String,
        context: Option<String>,
        response: String, 
    },
    WorkerStatus { name: String, health: WorkerHealth },
    LogDetected { 
        source: String, 
        level: LogLevel, 
        message: String,
        source_file: Option<PathBuf>,
        raw_lines: Option<String>,
    },
    FileChanged { path: String },
    RagSearch { query: String, request_id: Uuid },
    RagSearchResult { request_id: Uuid, query: String, results: Vec<String> },
    RagReindexComplete { project: String, count: u64, files_indexed: u64 },
    BuildRequested { project: String, command: String },
    BuildFinished { 
        project: String, 
        success: bool, 
        logs: String, 
        output: String, 
        duration_ms: u64 
    },
    TelegramCommand { 
        text: String, 
        chat_id: i64, 
        command: Option<String>, 
        args: Vec<String> 
    },
    FixApproved { alert_id: String },
}
