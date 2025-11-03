use todo_rs::{Priority, Todo, TodoList};

fn main() {
    println!("=== Todo.txt クレート デモ ===\n");

    // 基本的なタスクの作成
    println!("1. 基本的なタスクの作成:");
    let mut todo1 = Todo::new("Call Mom");
    println!("   {}", todo1);

    // 優先度付きタスク
    println!("\n2. 優先度付きタスク:");
    let todo2 = Todo::new("Submit report").with_priority(Priority::new('A').unwrap());
    println!("   {}", todo2);

    // コンテキストとプロジェクト付きタスク
    println!("\n3. コンテキストとプロジェクト付きタスク:");
    let mut todo3 = Todo::new("Review pull request");
    todo3.add_context("computer");
    todo3.add_project("Development");
    todo3.add_tag("due", "2024-11-15");
    println!("   {}", todo3);

    // 完了タスク
    println!("\n4. タスクの完了:");
    todo1.complete();
    println!("   {}", todo1);

    // パース
    println!("\n5. 文字列からのパース:");
    let parsed: Todo = "(B) 2024-11-01 Buy groceries +Shopping @store"
        .parse()
        .unwrap();
    println!("   元の文字列: (B) 2024-11-01 Buy groceries +Shopping @store");
    println!("   説明: {}", parsed.description);
    println!("   優先度: {:?}", parsed.priority);
    println!("   プロジェクト: {:?}", parsed.projects);
    println!("   コンテキスト: {:?}", parsed.contexts);

    // TodoList の操作
    println!("\n6. TodoList の操作:");
    let mut list = TodoList::new();
    list.add(Todo::new("Task A").with_priority(Priority::new('A').unwrap()));
    list.add(Todo::new("Task C").with_priority(Priority::new('C').unwrap()));
    list.add(Todo::new("Task B").with_priority(Priority::new('B').unwrap()));

    println!("   追加前:");
    for (i, todo) in list.iter().enumerate() {
        println!("     {}. {}", i + 1, todo);
    }

    list.sort_by_priority();
    println!("\n   優先度でソート後:");
    for (i, todo) in list.iter().enumerate() {
        println!("     {}. {}", i + 1, todo);
    }

    // フィルタリング
    println!("\n7. フィルタリング:");
    let mut list2 = TodoList::new();
    let mut work1 = Todo::new("Morning standup");
    work1.add_project("Work");
    list2.add(work1);

    let mut work2 = Todo::new("Code review");
    work2.add_project("Work");
    work2.complete();
    list2.add(work2);

    let mut personal = Todo::new("Exercise");
    personal.add_project("Health");
    list2.add(personal);

    let work_tasks = list2.with_project("Work");
    println!("   Work プロジェクトのタスク: {} 件", work_tasks.len());
    for todo in work_tasks {
        println!("     - {}", todo);
    }

    let incomplete = list2.incomplete();
    println!("\n   未完了タスク: {} 件", incomplete.len());
    for todo in incomplete {
        println!("     - {}", todo);
    }

    println!("\n=== デモ完了 ===");
}
