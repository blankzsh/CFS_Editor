use thiserror::Error;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    DatabaseError(String),

    #[error("JSON解析错误: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),

    #[error("图片处理错误: {0}")]
    ImageError(#[from] image::ImageError),

    #[error("无效输入: {0}")]
    InvalidInput(String),

    #[error("未找到记录: {0}")]
    NotFound(String),

    #[error("未加载数据库")]
    DatabaseNotLoaded,

    #[error("未选择球队")]
    TeamNotSelected,

    #[error("未知错误: {0}")]
    Unknown(String),

    #[error("SQLite错误: {0}")]
    SqliteError(#[from] rusqlite::Error),
}

pub type Result<T> = std::result::Result<T, AppError>; 