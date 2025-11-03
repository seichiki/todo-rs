# todo-rs

[![CI](https://github.com/seichiki/todo-rs/workflows/CI/badge.svg)](https://github.com/seichiki/todo-rs/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](README.md#license)

[Todo.txt](https://github.com/todotxt/todo.txt) フォーマットを Rust で簡単に操作できるクレートです。

## 機能

- ✅ **パース機能**: Todo.txt ファイルを読み込んで構造化データに変換
- ✅ **シリアライズ機能**: Rust の構造体から Todo.txt 形式に出力
- ✅ **CRUD 操作**: タスクの追加・削除・更新・検索
- ✅ **フィルタリング**: 優先度、プロジェクト、コンテキストでフィルタ
- ✅ **ソート機能**: 優先度や日付でソート

## インストール

### GitHub から直接インストール

現在は GitHub リポジトリから直接インストールできます：

```toml
[dependencies]
todo-rs = "https://github.com/seichiki/todo-rs"
```

### crates.io から（準備中）

将来的には crates.io に公開予定です：

```toml
[dependencies]
todo-rs = "0.1.0"
```

## 使い方

### 基本的な使い方

```rust
use todo_rs::{Todo, Priority};

// 新しいタスクを作成
let mut todo = Todo::new("Call Mom");

// 優先度を設定
let todo = todo.with_priority(Priority::new('A').unwrap());

// コンテキストとプロジェクトを追加
todo.add_context("phone");
todo.add_project("Family");

// タスクを完了としてマーク
todo.complete();

// Todo.txt 形式の文字列に変換
println!("{}", todo);
// 出力: x 2024-11-03 Call Mom +Family @phone
```

### 文字列からのパース

```rust
use todo_rs::Todo;

let todo: Todo = "(A) 2024-11-01 Call Mom +Family @phone".parse().unwrap();

assert_eq!(todo.description, "Call Mom");
assert!(todo.has_project("Family"));
assert!(todo.has_context("phone"));
```

### TodoList でファイル操作

```rust
use todo_rs::{TodoList, Todo, Priority};

// ファイルから読み込み
let mut list = TodoList::from_file("todo.txt").unwrap();

// 新しいタスクを追加
let todo = Todo::new("Buy groceries")
    .with_priority(Priority::new('B').unwrap());
list.add(todo);

// フィルタリング
let high_priority = list.with_priority(Priority::new('A').unwrap());
let incomplete = list.incomplete();
let work_tasks = list.with_project("Work");

// ソート
list.sort_by_priority();

// ファイルに保存
list.save_to_file("todo.txt").unwrap();
```

### フィルタリングとソート

```rust
use todo_rs::{TodoList, Priority};

let mut list = TodoList::from_file("todo.txt").unwrap();

// 優先度でソート（A -> B -> C）
list.sort_by_priority();

// 作成日でソート（新しい順）
list.sort_by_creation_date();

// 説明でソート（辞書順）
list.sort_by_description();

// カスタムフィルタ
let urgent = list.filter(|todo| {
    todo.priority == Some(Priority::new('A').unwrap()) && !todo.completed
});

// 特定のプロジェクトの未完了タスク
let work_incomplete: Vec<_> = list
    .with_project("Work")
    .into_iter()
    .filter(|todo| !todo.completed)
    .collect();
```

### CRUD 操作

```rust
use todo_rs::{TodoList, Todo};

let mut list = TodoList::new();

// Create: タスクを追加
list.add(Todo::new("Task 1"));
list.add(Todo::new("Task 2"));

// Read: タスクを取得
if let Some(todo) = list.get(0) {
    println!("{}", todo);
}

// Update: タスクを更新
if let Some(todo) = list.get_mut(0) {
    todo.complete();
    todo.add_tag("note", "completed-early");
}

// Delete: タスクを削除
let removed = list.remove(1).unwrap();
```

## Todo.txt フォーマットのサポート

このクレートは [公式 Todo.txt フォーマット仕様](https://github.com/todotxt/todo.txt) に準拠しています:

- ✅ 優先度: `(A)` から `(Z)`
- ✅ 日付: `YYYY-MM-DD` 形式
- ✅ 完了マーカー: `x` + 完了日
- ✅ プロジェクト: `+ProjectName`
- ✅ コンテキスト: `@context`
- ✅ カスタムタグ: `key:value`

### フォーマット例

```
(A) 2024-11-01 Call Mom +Family @phone
(B) Schedule meeting +Work @office due:2024-11-10
x 2024-11-03 2024-11-01 Buy milk +Shopping @store
Learn Rust programming +Learning @computer
```

## ライセンス

MIT OR Apache-2.0
