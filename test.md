### Code Quality Standards

This project maintains strict code quality requirements:

```bash
# All must pass before committing:
cargo build                                              # No warnings
cargo build --release                                    # No warnings
cargo clippy --all-targets --all-features -- -D warnings # No errors
cargo test                                               # All tests pass
```

See `CLAUDE.md` for comprehensive development guidelines and best practices.

```rust
/// List available syntax highlighting themes
pub fn list_themes() -> Vec<String> {
    let theme_set = ThemeSet::load_defaults();
    let mut themes: Vec<String> = theme_set.themes.keys().cloned().collect();
    themes.sort();
    themes
}
```
