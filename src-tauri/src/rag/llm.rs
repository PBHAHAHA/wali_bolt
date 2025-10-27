use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    input: ChatInput,
    parameters: Option<Parameters>,
}

#[derive(Debug, Serialize)]
struct ChatInput {
    messages: Vec<ChatMessage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
struct Parameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    output: ChatOutput,
    #[allow(dead_code)]
    usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
struct ChatOutput {
    text: Option<String>,
    choices: Option<Vec<Choice>>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ChatMessage,
}

#[derive(Debug, Deserialize)]
struct Usage {
    #[allow(dead_code)]
    input_tokens: usize,
    #[allow(dead_code)]
    output_tokens: usize,
}

/// LLM 服务
pub struct LLMService {
    client: Client,
    api_key: String,
    model: String,
}

impl LLMService {
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
    
    /// 生成回答
    pub async fn generate(&self, messages: Vec<ChatMessage>) -> Result<String> {
        let request = ChatRequest {
            model: self.model.clone(),
            input: ChatInput { messages },
            parameters: Some(Parameters {
                temperature: Some(0.7),
                top_p: Some(0.9),
            }),
        };
        
        let response = self.client
            .post("https://dashscope.aliyuncs.com/api/v1/services/aigc/text-generation/generation")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            anyhow::bail!("LLM API 请求失败: {} - {}", status, text);
        }
        
        let result: ChatResponse = response.json().await?;
        
        // 处理不同的响应格式
        if let Some(text) = result.output.text {
            return Ok(text);
        }
        
        if let Some(choices) = result.output.choices {
            if let Some(first_choice) = choices.first() {
                return Ok(first_choice.message.content.clone());
            }
        }
        
        anyhow::bail!("LLM API 返回空内容");
    }
    
    /// RAG 问答
    pub async fn answer_with_context(&self, question: &str, context: &str) -> Result<String> {
        let system_message = ChatMessage {
            role: "system".to_string(),
            content: "你是一个专业的知识库助手。请基于提供的文档内容回答用户问题。如果文档中没有相关信息，请诚实告知。".to_string(),
        };
        
        let user_message = ChatMessage {
            role: "user".to_string(),
            content: format!(
                "参考文档：\n\n{}\n\n问题：{}",
                context, question
            ),
        };
        
        self.generate(vec![system_message, user_message]).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    #[ignore] // 需要 API Key 才能运行
    async fn test_generate() {
        let api_key = std::env::var("QWEN_API_KEY").expect("需要设置 QWEN_API_KEY 环境变量");
        let service = LLMService::new(api_key, "qwen-turbo".to_string());
        
        let messages = vec![
            ChatMessage {
                role: "user".to_string(),
                content: "你好，请介绍一下你自己".to_string(),
            }
        ];
        
        let response = service.generate(messages).await.unwrap();
        
        assert!(!response.is_empty());
        println!("LLM 回复: {}", response);
    }
    
    #[tokio::test]
    #[ignore]
    async fn test_answer_with_context() {
        let api_key = std::env::var("QWEN_API_KEY").expect("需要设置 QWEN_API_KEY 环境变量");
        let service = LLMService::new(api_key, "qwen-turbo".to_string());
        
        let context = "苹果富含维生素C，对人体健康非常有益。每天吃一个苹果可以增强免疫力。";
        let question = "吃苹果有什么好处？";
        
        let answer = service.answer_with_context(question, context).await.unwrap();
        
        assert!(!answer.is_empty());
        println!("答案: {}", answer);
    }
}

