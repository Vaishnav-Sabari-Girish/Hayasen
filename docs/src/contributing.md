# Contributing to Hayasen

Contributions are welcome for the `hayasen` project! To ensure a smooth and effective collaboration, please follow these guidelines.

## Getting Started

1.  **Fork the repository:** Start by forking the `hayasen` repository to your GitHub account.
2.  **Clone your fork:** Clone your forked repository to your local machine:
    ```bash
    git clone https://github.com/Vaishnav-Sabari-Girish/hayasen.git
    cd hayasen
    ```
3.  **Create a new branch:** For each new feature or bug fix, create a new branch:
    ```bash
    git checkout -b feature/your-feature-name # For new features
    git checkout -b bugfix/your-bug-name    # For bug fixes
    ```

## Code Style Guidelines (Rust)

Please adhere to the existing code style and conventions of the project, as enforced by `cargo fmt` and `cargo clippy`.

*   **Imports:** Prefer `use crate::module::item;` or `use super::item;` for internal modules. Group related imports.
*   **Formatting:** Adhere to `cargo fmt` standards. You can run `cargo fmt` to automatically format your code.
*   **Naming Conventions:**
    *   `snake_case` for functions, variables, and modules.
    *   `PascalCase` for types (structs, enums, traits).
    *   `SCREAMING_SNAKE_CASE` for constants.
*   **Error Handling:** Utilize `Result` and `Option` types for explicit error handling. Avoid `unwrap()` and `expect()` in production code.
*   **Types:** Use explicit types where clarity is enhanced, otherwise let type inference work.
*   **Comments:** Explain *why* complex logic is implemented, not *what* it does.

## Development Workflow

### Building the Project

*   **Build:** `cargo build`
*   **Build (release):** `cargo build --release`

### Testing

*   **Run all tests:** `cargo test`
*   **Run a single test:** `cargo test -- <test_name>`

### Linting and Type Checking

*   **Linting:** `cargo clippy -- -D warnings`
*   **Check types:** `cargo check`

## Submitting Changes

1.  **Commit your changes:** Write clear, concise commit messages that explain the purpose of your changes.
2.  **Push to your fork:**
    ```bash
    git push origin feature/your-feature-name
    ```
3.  **Create a Pull Request:**
    *   Go to the original `hayasen` repository on GitHub.
    *   Click on "New Pull Request".
    *   Provide a descriptive title and a detailed explanation of your changes.
    *   Ensure all tests pass and linting checks are clear.

Thank you for contributing to Hayasen!
