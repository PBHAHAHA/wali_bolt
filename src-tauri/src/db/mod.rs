use sqlx::sqlite::{SqlitePool, SqlitePoolOptions, SqliteConnectOptions};
use std::path::PathBuf;
use anyhow::Result;
use std::str::FromStr;

pub mod models;

/// 数据库连接池
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// 初始化数据库
    pub async fn new(data_dir: &PathBuf) -> Result<Self> {
        // 确保数据目录存在
        tokio::fs::create_dir_all(data_dir).await?;
        
        let db_path = data_dir.join("wali.db");
        
        // 创建连接选项
        let options = SqliteConnectOptions::from_str(&format!("sqlite:///{}", db_path.to_string_lossy()))?
            .create_if_missing(true);
        
        // 创建连接池
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;
        
        // 运行迁移
        sqlx::query(include_str!("../../migrations/001_initial.sql"))
            .execute(&pool)
            .await?;
        
        Ok(Self { pool })
    }
    
    /// 获取连接池引用
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}

