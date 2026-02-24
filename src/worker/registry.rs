use crate::worker::worker_trait::Worker;
use std::collections::HashMap;
use std::sync::Arc;

pub struct WorkerRegistry {
    workers: HashMap<String, Arc<dyn Worker>>,
}

impl WorkerRegistry {
    pub fn new() -> Self {
        Self {
            workers: HashMap::new(),
        }
    }

    pub fn register(&mut self, worker: Arc<dyn Worker>) {
        self.workers.insert(worker.name().into(), worker);
    }

    pub fn get(&self, name: &str) -> Option<&Arc<dyn Worker>> {
        self.workers.get(name)
    }

    pub fn names(&self) -> Vec<String> {
        self.workers.keys().cloned().collect()
    }
}

impl Default for WorkerRegistry { fn default() -> Self { Self::new() } }
