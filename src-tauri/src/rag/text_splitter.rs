/// 文本分块器
pub struct TextSplitter {
    chunk_size: usize,
    chunk_overlap: usize,
}

impl TextSplitter {
    pub fn new(chunk_size: usize, chunk_overlap: usize) -> Self {
        Self {
            chunk_size,
            chunk_overlap,
        }
    }
    
    /// 将文本分割成多个块
    pub fn split(&self, text: &str) -> Vec<String> {
        if text.is_empty() {
            return Vec::new();
        }
        
        let chars: Vec<char> = text.chars().collect();
        let total_len = chars.len();
        
        if total_len <= self.chunk_size {
            return vec![text.to_string()];
        }
        
        let mut chunks = Vec::new();
        let mut start = 0;
        
        while start < total_len {
            let end = (start + self.chunk_size).min(total_len);
            let chunk: String = chars[start..end].iter().collect();
            chunks.push(chunk);
            
            if end >= total_len {
                break;
            }
            
            // 下一个块的起始位置，考虑重叠
            start = end - self.chunk_overlap;
        }
        
        chunks
    }
    
    /// 按段落分割
    pub fn split_by_paragraphs(&self, text: &str) -> Vec<String> {
        text.split("\n\n")
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.to_string())
            .collect()
    }
    
    /// 智能分割：先按段落，如果段落太长再按固定长度
    pub fn split_smart(&self, text: &str) -> Vec<String> {
        let paragraphs = self.split_by_paragraphs(text);
        let mut result = Vec::new();
        
        for para in paragraphs {
            if para.chars().count() <= self.chunk_size {
                result.push(para);
            } else {
                // 段落太长，按固定长度分割
                let sub_chunks = self.split(&para);
                result.extend(sub_chunks);
            }
        }
        
        result
    }
}

impl Default for TextSplitter {
    fn default() -> Self {
        Self::new(500, 50)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_split() {
        let splitter = TextSplitter::new(10, 2);
        let text = "这是一个测试文本用于验证分块功能是否正常工作";
        let chunks = splitter.split(text);
        
        assert!(chunks.len() > 1);
        println!("分块数量: {}", chunks.len());
        for (i, chunk) in chunks.iter().enumerate() {
            println!("块 {}: {}", i + 1, chunk);
        }
    }
    
    #[test]
    fn test_split_by_paragraphs() {
        let splitter = TextSplitter::default();
        let text = "第一段内容。\n\n第二段内容。\n\n第三段内容。";
        let chunks = splitter.split_by_paragraphs(text);
        
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0], "第一段内容。");
        assert_eq!(chunks[1], "第二段内容。");
        assert_eq!(chunks[2], "第三段内容。");
    }
    
    #[test]
    fn test_split_smart() {
        let splitter = TextSplitter::new(20, 5);
        let text = "短段落。\n\n这是一个非常非常非常非常非常非常非常非常长的段落需要分割。";
        let chunks = splitter.split_smart(text);
        
        assert!(chunks.len() >= 2);
        for (i, chunk) in chunks.iter().enumerate() {
            println!("智能分块 {}: {}", i + 1, chunk);
        }
    }
}

