pub mod embedding;
pub mod vector_store;
pub mod llm;
pub mod text_splitter;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RAGConfig {
    pub qwen_api_key: String,
    pub embedding_model: String,
    pub llm_model: String,
    pub chunk_size: usize,
    pub chunk_overlap: usize,
    pub top_k: usize,
}

impl Default for RAGConfig {
    fn default() -> Self {
        Self {
            qwen_api_key: String::new(),
            embedding_model: "text-embedding-v2".to_string(),
            llm_model: "qwen-turbo".to_string(),
            chunk_size: 800,
            chunk_overlap: 80,
            top_k: 3,
        }
    }
}

