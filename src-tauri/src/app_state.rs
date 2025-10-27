use crate::db::Database;
use crate::rag::{embedding::EmbeddingService, llm::LLMService, vector_store::VectorStore, RAGConfig};
use std::sync::{Arc, Mutex};
use anyhow::Result;

/// 应用全局状态
pub struct AppState {
    pub db: Database,
    pub vector_store: Arc<Mutex<VectorStore>>,
    pub embedding_service: Arc<Mutex<Option<EmbeddingService>>>,
    pub llm_service: Arc<Mutex<Option<LLMService>>>,
    pub rag_config: Arc<Mutex<RAGConfig>>,
}

impl AppState {
    pub async fn new(db: Database) -> Result<Self> {
        Ok(Self {
            db,
            vector_store: Arc::new(Mutex::new(VectorStore::new())),
            embedding_service: Arc::new(Mutex::new(None)),
            llm_service: Arc::new(Mutex::new(None)),
            rag_config: Arc::new(Mutex::new(RAGConfig::default())),
        })
    }
    
    /// 初始化 RAG 服务
    pub fn init_rag_services(&self, api_key: String) {
        let config = self.rag_config.lock().unwrap();
        
        let embedding_service = EmbeddingService::new(
            api_key.clone(),
            config.embedding_model.clone(),
        );
        
        let llm_service = LLMService::new(
            api_key,
            config.llm_model.clone(),
        );
        
        *self.embedding_service.lock().unwrap() = Some(embedding_service);
        *self.llm_service.lock().unwrap() = Some(llm_service);
    }
    
    /// 检查 RAG 服务是否已初始化
    pub fn is_rag_initialized(&self) -> bool {
        self.embedding_service.lock().unwrap().is_some()
            && self.llm_service.lock().unwrap().is_some()
    }
}

