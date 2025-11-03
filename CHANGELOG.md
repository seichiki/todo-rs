# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-11-03

### Added
- Initial release
- Todo.txt format parser
- Todo struct with support for:
  - Priority (A-Z)
  - Completion status
  - Creation and completion dates
  - Projects (+project)
  - Contexts (@context)
  - Custom tags (key:value)
- TodoList for managing multiple tasks
- File I/O operations (read/write todo.txt files)
- CRUD operations (Create, Read, Update, Delete)
- Filtering capabilities:
  - By completion status
  - By priority
  - By project
  - By context
  - Custom filters
- Sorting capabilities:
  - By priority
  - By creation date
  - By description
  - Custom sorting
- Comprehensive test suite (32 unit tests)
- Example usage in `examples/demo.rs`
- Full documentation with usage examples

[0.1.0]: https://github.com/seichiki/todo-rs/releases/tag/v0.1.0
