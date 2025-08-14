use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::store::{MemoryId, Result};

/// Result of a search in a [`MemoryIndex`].
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SearchResult {
    /// Identifier of the found item.
    pub id: MemoryId,
    /// Similarity score.
    pub score: f32,
}

/// Interface for vector or full text indices.
pub trait MemoryIndex {
    /// Adds an embedding vector associated with an item identifier.
    fn add_embedding(&mut self, id: &MemoryId, vector: Vec<f32>) -> Result<()>;
    /// Searches the index returning the top `k` most similar vectors.
    fn search(&self, query: Vec<f32>, k: usize) -> Result<Vec<SearchResult>>;
}

/// Naive in-memory implementation of [`MemoryIndex`].
#[derive(Default)]
pub struct InMemoryIndex {
    vectors: HashMap<MemoryId, Vec<f32>>,
}

impl InMemoryIndex {
    /// Creates a new empty index.
    pub fn new() -> Self {
        Self::default()
    }
}

impl MemoryIndex for InMemoryIndex {
    fn add_embedding(&mut self, id: &MemoryId, vector: Vec<f32>) -> Result<()> {
        self.vectors.insert(*id, vector);
        Ok(())
    }

    fn search(&self, query: Vec<f32>, k: usize) -> Result<Vec<SearchResult>> {
        let mut results: Vec<SearchResult> = self
            .vectors
            .iter()
            .map(|(id, v)| {
                let score = cosine_similarity(&query, v);
                SearchResult { id: *id, score }
            })
            .collect();
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results.truncate(k);
        Ok(results)
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a * norm_b)
    }
}
