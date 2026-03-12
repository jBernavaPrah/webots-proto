# Contributing to Webots Proto

Thank you for your interest in contributing!

## Development Workflow

1.  **Setup**: Ensure you have stable Rust installed (1.85+).
2.  **Test**: Run `cargo test` to ensure everything is working.
3.  **Format**: Run `cargo fmt` before submitting.
4.  **Lint**: Run `cargo clippy --all-targets` and fix warnings.

## Guidelines

-   **Code Style**: Use `cargo fmt`.
-   **Naming**: Use descriptive variable names. Avoid short names unless they are standard (e.g., `i`, `x`).
-   **Comments**: Add comments to explain your code, especially complex logic.
-   **Tests**: Add tests for new functionality. Do not add unit tests for actors unless explicitly requested.
-   **Documentation**: Update documentation if you change public APIs.
-   **Builder Pattern**: Prefer builder-style APIs with `::new` and `with_*` setters.

## PR Process

1.  Create a branch for your feature.
2.  Submit a PR with a clear description.
3.  Address review comments.
