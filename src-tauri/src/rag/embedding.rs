use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct EmbeddingRequest {
    model: String,
    input: InputData,
}

#[derive(Debug, Serialize)]
struct InputData {
    texts: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct EmbeddingResponse {
    output: Output,
    #[allow(dead_code)]
    usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
struct Output {
    embeddings: Vec<EmbeddingData>,
}

#[derive(Debug, Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
    #[allow(dead_code)]
    text_index: usize,
}

#[derive(Debug, Deserialize)]
struct Usage {
    #[allow(dead_code)]
    total_tokens: usize,
}

/// Embedding 服务
pub struct EmbeddingService {
    client: Client,
    api_key: String,
    model: String,
}

impl EmbeddingService {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model,
        }
    }
    
    pub fn api_key(&self) -> &String {
        &self.api_key
    }
    
    pub fn model(&self) -> &String {
        &self.model
    }
    
    /// 将文本转换为向量
    pub async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let request = EmbeddingRequest {
            model: self.model.clone(),
            input: InputData {
                texts: vec![text.to_string()],
            },
        };
        
        let response = self.client
            .post("https://dashscope.aliyuncs.com/api/v1/services/embeddings/text-embedding/text-embedding")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            anyhow::bail!("Embedding API 请求失败: {} - {}", status, text);
        }
        
        let result: EmbeddingResponse = response.json().await?;
        
        if result.output.embeddings.is_empty() {
            anyhow::bail!("Embedding API 返回空向量");
        }
        
        Ok(result.output.embeddings[0].embedding.clone())
    }
    
    /// 批量生成向量
    pub async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }
        
        let request = EmbeddingRequest {
            model: self.model.clone(),
            input: InputData {
                texts: texts.to_vec(),
            },
        };
        
        let response = self.client
            .post("https://dashscope.aliyuncs.com/api/v1/services/embeddings/text-embedding/text-embedding")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            anyhow::bail!("Embedding API 批量请求失败: {} - {}", status, text);
        }
        
        let result: EmbeddingResponse = response.json().await?;
        
        Ok(result.output.embeddings
            .into_iter()
            .map(|e| e.embedding)
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    #[ignore] // 需要 API Key 才能运行
    async fn test_embed() {
        let api_key = std::env::var("QWEN_API_KEY").expect("需要设置 QWEN_API_KEY 环境变量");
        let service = EmbeddingService::new(api_key, "text-embedding-v2".to_string());
        
        let embedding = service.embed("你好世界").await.unwrap();
        
        assert!(!embedding.is_empty());
        println!("向量维度: {}", embedding.len());
        println!("前5个值: {:?}", &embedding[..5]);
    }
}

