use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::ai::memory::persistent_store::{save_to_file, load_from_file, exists};
use crate::ai::memory::embeddings::embed_text;

const EMBEDDING_DIR: &str = "axon_state/embeddings";

/// Stores embeddings for one chat session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationEmbeddingStore {
    pub session_id: Uuid,
    pub vectors: HashMap<usize, Vec<f32>>, // message_index â†’ embedding
}

impl ConversationEmbeddingStore {

    fn ensure_dir() -> Result<()> {
        if !PathBuf::from(EMBEDDING_DIR).exists() {
            std::fs::create_dir_all(EMBEDDING_DIR)?;
        }
        Ok(())
    }

    fn path(session_id: Uuid) -> PathBuf {
        PathBuf::from(EMBEDDING_DIR)
            .join(format!("{}.json", session_id))
    }

    pub fn new(session_id: Uuid) -> Self {
        Self {
            session_id,
            vectors: HashMap::new(),
        }
    }

    pub fn save(&self) -> Result<()> {
        Self::ensure_dir()?;
        let path = Self::path(self.session_id);
        save_to_file(&path, self)
            .context("Failed saving conversation embeddings")
    }

    pub fn load(session_id: Uuid) -> Result<Self> {
        let path = Self::path(session_id);

        if !exists(&path) {
            return Ok(Self::new(session_id));
        }

        load_from_file(&path)
            .context("Failed loading conversation embeddings")
    }

    /// Generate and store embedding for a message
    pub async fn embed_message(
        &mut self,
        message_index: usize,
        text: &str,
    ) -> Result<()> {

        if self.vectors.contains_key(&message_index) {
            return Ok(()); // already embedded
        }

        if let Some(vec) = embed_text(text).await {
            self.vectors.insert(message_index, vec);
        }

        Ok(())
    }

    /// Simple cosine similarity
    pub fn search(
        &self,
        query_embedding: &[f32],
        top_k: usize,
    ) -> Vec<(usize, f32)> {

        let mut results: Vec<(usize, f32)> = self.vectors
            .iter()
            .map(|(idx, vec)| {
                (*idx, cosine_similarity(query_embedding, vec))
            })
            .collect();

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        results.into_iter().take(top_k).collect()
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for (x, y) in a.iter().zip(b.iter()) {
        dot += x * y;
        norm_a += x * x;
        norm_b += y * y;
    }

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot / (norm_a.sqrt() * norm_b.sqrt())
}



