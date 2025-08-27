# CRUSH.md

This file outlines the build, lint, test, and code style guidelines for the `hayasen` project.

## Commands

- **Build:** `cargo build`
- **Build (release):** `cargo build --release`
- **Run tests:** `cargo test`
- **Run a single test:** `cargo test -- <test_name>`
- **Linting:** `cargo clippy -- -D warnings`
- **Formatting:** `cargo fmt`
- **Check types:** `cargo check`

## Code Style Guidelines (Rust)

- **Imports:** Prefer `use crate::module::item;` or `use super::item;` for internal modules. Group related imports.
- **Formatting:** Adhere to `cargo fmt` standards.
- **Naming Conventions:**
    - `snake_case` for functions, variables, and modules.
    - `PascalCase` for types (structs, enums, traits).
    - `SCREAMING_SNAKE_CASE` for constants.
- **Error Handling:** Utilize `Result` and `Option` types for explicit error handling. Avoid `unwrap()` and `expect()` in production code.
- **Types:** Use explicit types where clarity is enhanced, otherwise let type inference work.
- **Comments:** Explain _why_ complex logic is implemented, not _what_ it does.
