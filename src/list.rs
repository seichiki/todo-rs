use crate::{Result, Todo, TodoError};
use std::fmt;
use std::fs;
use std::path::Path;

/// 複数の Todo タスクを管理するリスト
#[derive(Debug, Clone, Default)]
pub struct TodoList {
    todos: Vec<Todo>,
}

impl TodoList {
    /// 新しい空の TodoList を作成
    pub fn new() -> Self {
        Self { todos: Vec::new() }
    }

    /// ファイルから TodoList を読み込み
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        Self::from_string(&content)
    }

    /// 文字列から TodoList を作成
    pub fn from_string(content: &str) -> Result<Self> {
        let mut todos = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();

            // 空行はスキップ
            if line.is_empty() {
                continue;
            }

            match line.parse::<Todo>() {
                Ok(todo) => todos.push(todo),
                Err(e) => {
                    eprintln!("警告: {}行目のパースに失敗しました: {}", line_num + 1, e);
                }
            }
        }

        Ok(Self { todos })
    }

    /// TodoList をファイルに保存
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = self.to_string();
        fs::write(path, content)?;
        Ok(())
    }



    /// タスクを追加
    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    /// インデックスでタスクを取得
    pub fn get(&self, index: usize) -> Option<&Todo> {
        self.todos.get(index)
    }

    /// インデックスでタスクを可変参照で取得
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Todo> {
        self.todos.get_mut(index)
    }

    /// インデックスでタスクを削除
    pub fn remove(&mut self, index: usize) -> Result<Todo> {
        if index < self.todos.len() {
            Ok(self.todos.remove(index))
        } else {
            Err(TodoError::IndexOutOfBounds(index))
        }
    }

    /// すべてのタスクを取得
    pub fn all(&self) -> &[Todo] {
        &self.todos
    }

    /// すべてのタスクを可変参照で取得
    pub fn all_mut(&mut self) -> &mut [Todo] {
        &mut self.todos
    }

    /// タスクの数を取得
    pub fn len(&self) -> usize {
        self.todos.len()
    }

    /// リストが空かチェック
    pub fn is_empty(&self) -> bool {
        self.todos.is_empty()
    }

    /// イテレータを取得
    pub fn iter(&self) -> impl Iterator<Item = &Todo> {
        self.todos.iter()
    }

    /// 可変イテレータを取得
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Todo> {
        self.todos.iter_mut()
    }

    /// 条件に一致するタスクをフィルタリング
    pub fn filter<F>(&self, predicate: F) -> Vec<&Todo>
    where
        F: Fn(&Todo) -> bool,
    {
        self.todos.iter().filter(|todo| predicate(todo)).collect()
    }

    /// 完了していないタスクのみ取得
    pub fn incomplete(&self) -> Vec<&Todo> {
        self.filter(|todo| !todo.completed)
    }

    /// 完了したタスクのみ取得
    pub fn completed(&self) -> Vec<&Todo> {
        self.filter(|todo| todo.completed)
    }

    /// 特定の優先度のタスクを取得
    pub fn with_priority(&self, priority: crate::Priority) -> Vec<&Todo> {
        self.filter(|todo| todo.priority == Some(priority))
    }

    /// 特定のプロジェクトのタスクを取得
    pub fn with_project(&self, project: &str) -> Vec<&Todo> {
        self.filter(|todo| todo.has_project(project))
    }

    /// 特定のコンテキストのタスクを取得
    pub fn with_context(&self, context: &str) -> Vec<&Todo> {
        self.filter(|todo| todo.has_context(context))
    }

    /// タスクをソート
    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&Todo, &Todo) -> std::cmp::Ordering,
    {
        self.todos.sort_by(compare);
    }

    /// 優先度でソート（高い優先度が先）
    pub fn sort_by_priority(&mut self) {
        self.todos.sort_by(|a, b| match (a.priority, b.priority) {
            (Some(p1), Some(p2)) => p1.cmp(&p2),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        });
    }

    /// 作成日でソート（新しい順）
    pub fn sort_by_creation_date(&mut self) {
        self.todos
            .sort_by(|a, b| match (a.creation_date, b.creation_date) {
                (Some(d1), Some(d2)) => d2.cmp(&d1),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            });
    }

    /// 説明でソート（辞書順）
    pub fn sort_by_description(&mut self) {
        self.todos.sort_by(|a, b| a.description.cmp(&b.description));
    }
}

impl fmt::Display for TodoList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = self
            .todos
            .iter()
            .map(|todo| todo.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Priority;

    #[test]
    fn test_new_list() {
        let list = TodoList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_add_and_get() {
        let mut list = TodoList::new();
        let todo = Todo::new("Test task");
        list.add(todo.clone());

        assert_eq!(list.len(), 1);
        assert_eq!(list.get(0).unwrap().description, "Test task");
    }

    #[test]
    fn test_remove() {
        let mut list = TodoList::new();
        list.add(Todo::new("Task 1"));
        list.add(Todo::new("Task 2"));

        let removed = list.remove(0).unwrap();
        assert_eq!(removed.description, "Task 1");
        assert_eq!(list.len(), 1);
        assert_eq!(list.get(0).unwrap().description, "Task 2");
    }

    #[test]
    fn test_filter_completed() {
        let mut list = TodoList::new();
        let mut todo1 = Todo::new("Task 1");
        todo1.complete();
        list.add(todo1);
        list.add(Todo::new("Task 2"));

        let completed = list.completed();
        assert_eq!(completed.len(), 1);

        let incomplete = list.incomplete();
        assert_eq!(incomplete.len(), 1);
    }

    #[test]
    fn test_with_priority() {
        let mut list = TodoList::new();
        let priority_a = Priority::new('A').unwrap();
        list.add(Todo::new("Task 1").with_priority(priority_a));
        list.add(Todo::new("Task 2"));

        let high_priority = list.with_priority(priority_a);
        assert_eq!(high_priority.len(), 1);
    }

    #[test]
    fn test_sort_by_priority() {
        let mut list = TodoList::new();
        list.add(Todo::new("Task C").with_priority(Priority::new('C').unwrap()));
        list.add(Todo::new("Task A").with_priority(Priority::new('A').unwrap()));
        list.add(Todo::new("Task B").with_priority(Priority::new('B').unwrap()));

        list.sort_by_priority();

        assert_eq!(list.get(0).unwrap().priority.unwrap().as_char(), 'A');
        assert_eq!(list.get(1).unwrap().priority.unwrap().as_char(), 'B');
        assert_eq!(list.get(2).unwrap().priority.unwrap().as_char(), 'C');
    }

    #[test]
    fn test_from_string() {
        let content = "(A) Task 1\n(B) Task 2 +Project @context\nx 2024-11-03 Task 3";
        let list = TodoList::from_string(content).unwrap();

        assert_eq!(list.len(), 3);
        assert!(list.get(2).unwrap().completed);
    }

    #[test]
    fn test_to_string() {
        let mut list = TodoList::new();
        list.add(Todo::new("Task 1").with_priority(Priority::new('A').unwrap()));
        list.add(Todo::new("Task 2"));

        let output = list.to_string();
        assert!(output.contains("(A) Task 1"));
        assert!(output.contains("Task 2"));
    }
}
