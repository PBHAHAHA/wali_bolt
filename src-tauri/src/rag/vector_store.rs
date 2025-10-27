use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDocument {
    pub id: String,
    pub content: String,
    pub embedding: Vec<f32>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub document: VectorDocument,
    pub similarity: f32,
}

/// 内存向量存储
pub struct VectorStore {
    documents: Arc<Mutex<Vec<VectorDocument>>>,
}

impl VectorStore {
    pub fn new() -> Self {
        Self {
            documents: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    /// 添加文档
    pub fn add_document(&self, doc: VectorDocument) {
        let mut docs = self.documents.lock().unwrap();
        docs.push(doc);
    }
    
    /// 批量添加文档
    pub fn add_documents(&self, new_docs: Vec<VectorDocument>) {
        let mut docs = self.documents.lock().unwrap();
        docs.extend(new_docs);
    }
    
    /// 相似度搜索
    pub fn search(&self, query_embedding: &[f32], top_k: usize) -> Vec<SearchResult> {
        let docs = self.documents.lock().unwrap();
        
        if docs.is_empty() {
            return Vec::new();
        }
        
        let mut results: Vec<SearchResult> = docs
            .iter()
            .map(|doc| {
                let similarity = cosine_similarity(&doc.embedding, query_embedding);
                SearchResult {
                    document: doc.clone(),
                    similarity,
                }
            })
            .collect();
        
        // 按相似度降序排序
        results.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());
        
        // 返回 Top-K
        results.into_iter().take(top_k).collect()
    }
    
    /// 根据文档 ID 删除
    pub fn remove_by_document_id(&self, document_id: &str) {
        let mut docs = self.documents.lock().unwrap();
        docs.retain(|doc| {
            if let Some(id) = doc.metadata.get("document_id") {
                id.as_str() != Some(document_id)
            } else {
                true
            }
        });
    }
    
    /// 清空所有文档
    pub fn clear(&self) {
        let mut docs = self.documents.lock().unwrap();
        docs.clear();
    }
    
    /// 获取文档数量
    pub fn len(&self) -> usize {
        self.documents.lock().unwrap().len()
    }
    
    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.documents.lock().unwrap().is_empty()
    }
    
    /// 序列化保存
    pub fn serialize(&self) -> Result<String, serde_json::Error> {
        let docs = self.documents.lock().unwrap();
        serde_json::to_string(&*docs)
    }
    
    /// 从序列化数据加载
    pub fn deserialize(&self, data: &str) -> Result<(), serde_json::Error> {
        let docs: Vec<VectorDocument> = serde_json::from_str(data)?;
        let mut store_docs = self.documents.lock().unwrap();
        *store_docs = docs;
        Ok(())
    }
}

impl Default for VectorStore {
    fn default() -> Self {
        Self::new()
    }
}

/// 计算余弦相似度
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }
    
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let magnitude_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let magnitude_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    
    if magnitude_a == 0.0 || magnitude_b == 0.0 {
        return 0.0;
    }
    
    dot_product / (magnitude_a * magnitude_b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert_eq!(cosine_similarity(&a, &b), 1.0);
        
        let c = vec![0.0, 1.0, 0.0];
        assert_eq!(cosine_similarity(&a, &c), 0.0);
        
        let d = vec![0.5, 0.5, 0.0];
        let sim = cosine_similarity(&a, &d);
        assert!(sim > 0.7 && sim < 0.8);
    }
    
    #[test]
    fn test_vector_store() {
        let store = VectorStore::new();
        
        // 添加文档
        store.add_document(VectorDocument {
            id: "1".to_string(),
            content: "苹果很好吃".to_string(),
            embedding: vec![0.8, 0.2, 0.1],
            metadata: json!({"source": "doc1"}),
        });
        
        store.add_document(VectorDocument {
            id: "2".to_string(),
            content: "今天天气很好".to_string(),
            embedding: vec![0.1, 0.9, 0.2],
            metadata: json!({"source": "doc2"}),
        });
        
        // 搜索
        let query = vec![0.7, 0.3, 0.1];
        let results = store.search(&query, 2);
        
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].document.id, "1");
        assert!(results[0].similarity > results[1].similarity);
    }
}

