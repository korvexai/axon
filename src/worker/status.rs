use crate::event::event::WorkerHealth;

#[derive(Debug, Clone)]
pub struct WorkerStatus {
    pub health: WorkerHealth,
    pub task: String,
}

impl WorkerStatus {
    pub fn running(task: &str) -> Self {
        Self {
            health: WorkerHealth::Running,
            task: task.to_string(),
        }
    }

    pub fn idle() -> Self {
        Self {
            health: WorkerHealth::Idle,
            task: "idle".to_string(),
        }
    }

    pub fn error(msg: &str) -> Self {
        Self {
            health: WorkerHealth::Error(msg.to_string()),
            task: "error".to_string(),
        }
    }
}
