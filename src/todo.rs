use crate::priority::Priority;
use chrono::NaiveDate;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

/// Todo.txt のタスクを表す構造体
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Todo {
    /// タスクが完了しているかどうか
    pub completed: bool,

    /// 優先度 (A-Z)
    pub priority: Option<Priority>,

    /// 完了日 (完了している場合のみ)
    pub completion_date: Option<NaiveDate>,

    /// 作成日
    pub creation_date: Option<NaiveDate>,

    /// タスクの説明文
    pub description: String,

    /// コンテキスト (@で始まるタグ)
    pub contexts: Vec<String>,

    /// プロジェクト (+で始まるタグ)
    pub projects: Vec<String>,

    /// 追加のメタデータ (key:value 形式)
    pub tags: HashMap<String, String>,
}

impl Todo {
    /// 新しい未完了タスクを作成
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            completed: false,
            priority: None,
            completion_date: None,
            creation_date: None,
            description: description.into(),
            contexts: Vec::new(),
            projects: Vec::new(),
            tags: HashMap::new(),
        }
    }

    /// タスクを完了としてマーク
    pub fn complete(&mut self) {
        self.completed = true;
        if self.completion_date.is_none() {
            self.completion_date = Some(chrono::Local::now().naive_local().date());
        }
    }

    /// タスクを未完了としてマーク
    pub fn uncomplete(&mut self) {
        self.completed = false;
        self.completion_date = None;
    }

    /// 優先度を設定
    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = Some(priority);
        self
    }

    /// 作成日を設定
    pub fn with_creation_date(mut self, date: NaiveDate) -> Self {
        self.creation_date = Some(date);
        self
    }

    /// コンテキストを追加
    pub fn add_context(&mut self, context: impl Into<String>) {
        let ctx = context.into();
        if !self.contexts.contains(&ctx) {
            self.contexts.push(ctx);
        }
    }

    /// プロジェクトを追加
    pub fn add_project(&mut self, project: impl Into<String>) {
        let proj = project.into();
        if !self.projects.contains(&proj) {
            self.projects.push(proj);
        }
    }

    /// タグを追加
    pub fn add_tag(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.tags.insert(key.into(), value.into());
    }

    /// 特定のコンテキストを持つかチェック
    pub fn has_context(&self, context: &str) -> bool {
        self.contexts.iter().any(|c| c == context)
    }

    /// 特定のプロジェクトを持つかチェック
    pub fn has_project(&self, project: &str) -> bool {
        self.projects.iter().any(|p| p == project)
    }

    /// 特定のタグを持つかチェック
    pub fn has_tag(&self, key: &str) -> bool {
        self.tags.contains_key(key)
    }

    /// タグの値を取得
    pub fn get_tag(&self, key: &str) -> Option<&String> {
        self.tags.get(key)
    }
}

impl FromStr for Todo {
    type Err = crate::error::TodoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        crate::parser::parse_todo(s)
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 完了マーカー
        if self.completed {
            write!(f, "x")?;

            // 完了日
            if let Some(date) = self.completion_date {
                write!(f, " {}", date.format("%Y-%m-%d"))?;
            }

            // 作成日
            if let Some(date) = self.creation_date {
                write!(f, " {}", date.format("%Y-%m-%d"))?;
            }

            write!(f, " ")?;
        } else {
            // 優先度（未完了の場合のみ）
            if let Some(priority) = self.priority {
                write!(f, "{} ", priority)?;
            }

            // 作成日
            if let Some(date) = self.creation_date {
                write!(f, "{} ", date.format("%Y-%m-%d"))?;
            }
        }

        // 説明
        write!(f, "{}", self.description)?;

        // プロジェクト
        for project in &self.projects {
            write!(f, " +{}", project)?;
        }

        // コンテキスト
        for context in &self.contexts {
            write!(f, " @{}", context)?;
        }

        // タグ（key:value）
        let mut tags: Vec<_> = self.tags.iter().collect();
        tags.sort_by_key(|(k, _)| *k);
        for (key, value) in tags {
            write!(f, " {}:{}", key, value)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_todo() {
        let todo = Todo::new("Test task");
        assert!(!todo.completed);
        assert_eq!(todo.description, "Test task");
        assert!(todo.priority.is_none());
    }

    #[test]
    fn test_complete_todo() {
        let mut todo = Todo::new("Test task");
        todo.complete();
        assert!(todo.completed);
        assert!(todo.completion_date.is_some());
    }

    #[test]
    fn test_add_context_and_project() {
        let mut todo = Todo::new("Test task");
        todo.add_context("phone");
        todo.add_project("Work");

        assert!(todo.has_context("phone"));
        assert!(todo.has_project("Work"));
        assert!(!todo.has_context("email"));
    }

    #[test]
    fn test_with_priority() {
        let priority = Priority::new('A').unwrap();
        let todo = Todo::new("Test task").with_priority(priority);
        assert_eq!(todo.priority, Some(priority));
    }

    #[test]
    fn test_display_simple_todo() {
        let todo = Todo::new("Call Mom");
        assert_eq!(todo.to_string(), "Call Mom");
    }

    #[test]
    fn test_display_todo_with_priority() {
        let todo = Todo::new("Call Mom").with_priority(Priority::new('A').unwrap());
        assert_eq!(todo.to_string(), "(A) Call Mom");
    }

    #[test]
    fn test_display_todo_with_date() {
        let date = NaiveDate::from_ymd_opt(2024, 11, 3).unwrap();
        let todo = Todo::new("Call Mom").with_creation_date(date);
        assert_eq!(todo.to_string(), "2024-11-03 Call Mom");
    }

    #[test]
    fn test_display_completed_todo() {
        let mut todo = Todo::new("Call Mom");
        todo.completion_date = Some(NaiveDate::from_ymd_opt(2024, 11, 3).unwrap());
        todo.creation_date = Some(NaiveDate::from_ymd_opt(2024, 11, 1).unwrap());
        todo.completed = true;
        assert_eq!(todo.to_string(), "x 2024-11-03 2024-11-01 Call Mom");
    }

    #[test]
    fn test_display_todo_with_context_and_project() {
        let mut todo = Todo::new("Call Mom");
        todo.add_project("Family");
        todo.add_context("phone");
        assert_eq!(todo.to_string(), "Call Mom +Family @phone");
    }

    #[test]
    fn test_display_todo_with_tags() {
        let mut todo = Todo::new("Submit report");
        todo.add_tag("due", "2024-11-10");
        assert_eq!(todo.to_string(), "Submit report due:2024-11-10");
    }

    #[test]
    fn test_roundtrip_parse_and_display() {
        let original = "(A) 2024-11-01 Call Mom +Family @phone due:2024-11-10";
        let todo: Todo = original.parse().unwrap();
        let serialized = todo.to_string();

        // パースして再シリアライズした結果が元と一致する（順序は異なる可能性がある）
        assert!(serialized.contains("(A)"));
        assert!(serialized.contains("2024-11-01"));
        assert!(serialized.contains("Call Mom"));
        assert!(serialized.contains("+Family"));
        assert!(serialized.contains("@phone"));
        assert!(serialized.contains("due:2024-11-10"));
    }
}
