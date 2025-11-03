use crate::{Priority, Result, Todo, TodoError};
use chrono::NaiveDate;

/// Todo.txt 形式の文字列をパースする
pub fn parse_todo(line: &str) -> Result<Todo> {
    let line = line.trim();

    if line.is_empty() {
        return Err(TodoError::ParseError("空の行はパースできません".into()));
    }

    let mut todo = Todo::new("");
    let mut parts = line.split_whitespace().peekable();

    // 完了チェック
    if parts.peek() == Some(&"x") {
        todo.completed = true;
        parts.next();

        // 完了日のパース
        if let Some(&date_str) = parts.peek() {
            if let Ok(date) = parse_date(date_str) {
                todo.completion_date = Some(date);
                parts.next();
            }
        }
    }

    // 優先度のパース（未完了の場合のみ）
    if !todo.completed {
        if let Some(&priority_str) = parts.peek() {
            if priority_str.starts_with('(')
                && priority_str.ends_with(')')
                && priority_str.len() == 3
            {
                if let Ok(priority) = priority_str.parse::<Priority>() {
                    todo.priority = Some(priority);
                    parts.next();
                }
            }
        }
    }

    // 作成日のパース
    if let Some(&date_str) = parts.peek() {
        if let Ok(date) = parse_date(date_str) {
            todo.creation_date = Some(date);
            parts.next();
        }
    }

    // 残りの部分（説明、コンテキスト、プロジェクト、タグ）をパース
    let mut description_parts = Vec::new();

    for part in parts {
        if part.starts_with('@') && part.len() > 1 {
            // コンテキスト
            todo.contexts.push(part[1..].to_string());
        } else if part.starts_with('+') && part.len() > 1 {
            // プロジェクト
            todo.projects.push(part[1..].to_string());
        } else if part.contains(':') {
            // key:value タグ
            if let Some((key, value)) = part.split_once(':') {
                if !key.is_empty()
                    && !value.is_empty()
                    && !key.contains(char::is_whitespace)
                    && !value.contains(char::is_whitespace)
                {
                    todo.tags.insert(key.to_string(), value.to_string());
                    continue;
                }
            }
            // タグとして解釈できない場合は説明の一部として扱う
            description_parts.push(part);
        } else {
            // 通常のテキスト
            description_parts.push(part);
        }
    }

    todo.description = description_parts.join(" ");

    if todo.description.is_empty() && todo.contexts.is_empty() && todo.projects.is_empty() {
        return Err(TodoError::ParseError("タスクの内容が空です".into()));
    }

    Ok(todo)
}

/// YYYY-MM-DD 形式の日付をパースする
fn parse_date(s: &str) -> Result<NaiveDate> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| TodoError::InvalidDateFormat(s.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_todo() {
        let todo = parse_todo("Call Mom").unwrap();
        assert_eq!(todo.description, "Call Mom");
        assert!(!todo.completed);
        assert!(todo.priority.is_none());
    }

    #[test]
    fn test_parse_todo_with_priority() {
        let todo = parse_todo("(A) Call Mom").unwrap();
        assert_eq!(todo.description, "Call Mom");
        assert!(todo.priority.is_some());
        assert_eq!(todo.priority.unwrap().as_char(), 'A');
    }

    #[test]
    fn test_parse_todo_with_date() {
        let todo = parse_todo("2024-11-03 Call Mom").unwrap();
        assert_eq!(todo.description, "Call Mom");
        assert!(todo.creation_date.is_some());
    }

    #[test]
    fn test_parse_todo_with_priority_and_date() {
        let todo = parse_todo("(A) 2024-11-03 Call Mom").unwrap();
        assert_eq!(todo.description, "Call Mom");
        assert!(todo.priority.is_some());
        assert!(todo.creation_date.is_some());
    }

    #[test]
    fn test_parse_todo_with_context_and_project() {
        let todo = parse_todo("(A) Call Mom +Family @phone").unwrap();
        assert_eq!(todo.description, "Call Mom");
        assert!(todo.has_context("phone"));
        assert!(todo.has_project("Family"));
    }

    #[test]
    fn test_parse_completed_todo() {
        let todo = parse_todo("x 2024-11-03 2024-11-01 Call Mom").unwrap();
        assert!(todo.completed);
        assert!(todo.completion_date.is_some());
        assert!(todo.creation_date.is_some());
        assert_eq!(todo.description, "Call Mom");
    }

    #[test]
    fn test_parse_todo_with_tags() {
        let todo = parse_todo("(A) Submit report due:2024-11-10 +Work").unwrap();
        assert_eq!(todo.description, "Submit report");
        assert_eq!(todo.get_tag("due"), Some(&"2024-11-10".to_string()));
        assert!(todo.has_project("Work"));
    }

    #[test]
    fn test_parse_complex_todo() {
        let todo =
            parse_todo("(A) 2024-11-01 Call Mom +Family +PeaceLoveAndHappiness @iphone @phone")
                .unwrap();
        assert_eq!(todo.description, "Call Mom");
        assert_eq!(todo.priority.unwrap().as_char(), 'A');
        assert!(todo.creation_date.is_some());
        assert_eq!(todo.projects.len(), 2);
        assert_eq!(todo.contexts.len(), 2);
    }

    #[test]
    fn test_parse_email_in_description() {
        let todo = parse_todo("Email SoAndSo at soandso@example.com").unwrap();
        assert_eq!(todo.description, "Email SoAndSo at soandso@example.com");
        assert_eq!(todo.contexts.len(), 0);
    }

    #[test]
    fn test_parse_empty_line() {
        assert!(parse_todo("").is_err());
        assert!(parse_todo("   ").is_err());
    }
}
