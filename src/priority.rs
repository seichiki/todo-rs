use std::fmt;

/// タスクの優先度 (A-Z)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Priority(char);

impl Priority {
    /// 新しい優先度を作成 (A-Z のみ有効)
    pub fn new(c: char) -> Option<Self> {
        if c.is_ascii_uppercase() {
            Some(Priority(c))
        } else {
            None
        }
    }

    /// 優先度の文字を取得
    pub fn as_char(&self) -> char {
        self.0
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.0)
    }
}

impl std::str::FromStr for Priority {
    type Err = crate::error::TodoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 3 && s.starts_with('(') && s.ends_with(')') {
            let c = s.chars().nth(1).unwrap();
            Priority::new(c).ok_or_else(|| {
                crate::error::TodoError::InvalidPriority(format!(
                    "優先度は A-Z である必要があります: {}",
                    c
                ))
            })
        } else {
            Err(crate::error::TodoError::InvalidPriority(format!(
                "無効な優先度フォーマット: {}",
                s
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_new() {
        assert!(Priority::new('A').is_some());
        assert!(Priority::new('Z').is_some());
        assert!(Priority::new('a').is_none());
        assert!(Priority::new('1').is_none());
    }

    #[test]
    fn test_priority_display() {
        let priority = Priority::new('A').unwrap();
        assert_eq!(priority.to_string(), "(A)");
    }

    #[test]
    fn test_priority_ord() {
        let a = Priority::new('A').unwrap();
        let b = Priority::new('B').unwrap();
        assert!(a < b);
    }
}
