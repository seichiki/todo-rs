use thiserror::Error;

/// Todo.txt 操作時のエラー型
#[derive(Error, Debug)]
pub enum TodoError {
    #[error("パース中にエラーが発生しました: {0}")]
    ParseError(String),

    #[error("無効な日付フォーマット: {0}")]
    InvalidDateFormat(String),

    #[error("無効な優先度: {0}")]
    InvalidPriority(String),

    #[error("ファイル操作中にエラーが発生しました: {0}")]
    IoError(#[from] std::io::Error),

    #[error("指定されたインデックスが見つかりません: {0}")]
    IndexOutOfBounds(usize),
}

pub type Result<T> = std::result::Result<T, TodoError>;
