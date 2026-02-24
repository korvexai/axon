use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::ai::memory::persistent_store::{save_to_file, load_from_file, exists};

const JOB_DIR: &str = "axon_state/jobs";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobRecord {
    pub id: Uuid,
    pub job_type: String,
    pub status: JobStatus,
    pub last_step: Option<String>,
    pub payload: Option<String>,
}

pub struct JobStore;

impl JobStore {

    fn ensure_dir() -> Result<()> {
        if !PathBuf::from(JOB_DIR).exists() {
            fs::create_dir_all(JOB_DIR)?;
        }
        Ok(())
    }

    fn path(id: Uuid) -> PathBuf {
        PathBuf::from(JOB_DIR)
            .join(format!("{}.json", id))
    }

    /// Create new job and persist immediately
    pub fn create(job_type: &str, payload: Option<String>) -> Result<JobRecord> {
        Self::ensure_dir()?;

        let record = JobRecord {
            id: Uuid::new_v4(),
            job_type: job_type.to_string(),
            status: JobStatus::Pending,
            last_step: None,
            payload,
        };

        Self::save(&record)?;
        Ok(record)
    }

    pub fn save(job: &JobRecord) -> Result<()> {
        Self::ensure_dir()?;
        let path = Self::path(job.id);
        save_to_file(&path, job)
            .context("Failed saving job")
    }

    pub fn load(id: Uuid) -> Result<JobRecord> {
        let path = Self::path(id);

        if !exists(&path) {
            anyhow::bail!("Job not found");
        }

        load_from_file(&path)
            .context("Failed loading job")
    }

    /// Return all incomplete jobs (for resume)
    pub fn load_incomplete() -> Result<Vec<JobRecord>> {
        Self::ensure_dir()?;

        let mut jobs = Vec::new();

        for entry in fs::read_dir(JOB_DIR)? {
            let entry = entry?;
            let path = entry.path();

            if let Ok(job) = load_from_file::<JobRecord>(&path) {
                match job.status {
                    JobStatus::Pending | JobStatus::Running => {
                        jobs.push(job);
                    }
                    _ => {}
                }
            }
        }

        Ok(jobs)
    }

    pub fn update_status(job: &mut JobRecord, status: JobStatus) -> Result<()> {
        job.status = status;
        Self::save(job)
    }

    pub fn update_step(job: &mut JobRecord, step: &str) -> Result<()> {
        job.last_step = Some(step.to_string());
        Self::save(job)
    }
}



