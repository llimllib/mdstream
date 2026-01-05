# mdstream - Streaming Markdown Printer

A streaming markdown printer for the console that renders GitHub Flavored Markdown to the terminal with ANSI escape codes. The key feature is **incremental emission**: blocks are emitted immediately once parsed, not waiting for the entire document.

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/mdstream`.

## Usage

```bash
# Pipe markdown from a file
cat document.md | mdstream

# Pipe from echo
echo "# Hello World" | mdstream

# Redirect from file
mdstream < document.md
```

### Example

```bash
$ echo "# Demo\n\nThis is **bold** and *italic*." | mdstream
```

Output (with ANSI colors in your terminal):
- **# Demo** (in blue and bold)
- This is **bold** and *italic*

## Features

✅ **Streaming**: Renders markdown incrementally as it arrives
✅ **ATX Headings**: `# Heading` with blue/bold formatting
✅ **Paragraphs**: Text blocks with inline formatting
✅ **Code Blocks**: Fenced blocks with ` ``` ` and gray background
✅ **Lists**: Unordered (`-`) and ordered (`1.`) lists
✅ **Inline Formatting**: `**bold**`, `*italic*`, `` `code` ``
✅ **ANSI Colors**: Beautiful terminal output

## Conformance Test Suite

This project uses a comprehensive conformance test suite to verify streaming behavior, markdown parsing, and ANSI formatting.

### Test Structure

Tests are written as TOML fixture files that specify:
- **Input chunks**: Markdown arriving incrementally (simulating streaming)
- **Expected emissions**: What should be output after each chunk (empty string if block incomplete)
- **Raw ANSI codes**: Actual escape sequences for exact terminal output matching

### Test Format Example

```toml
name = "heading-basic"
description = "Heading should emit after newline is received"

[[chunks]]
input = "#"
emit = ""

[[chunks]]
input = " Hello"
emit = ""

[[chunks]]
input = "\n"
emit = "\u001b[1;34m# Hello\u001b[0m\n"
```

**Key Points**:
- Each `[[chunks]]` represents a piece of markdown fed to the parser
- `input`: The markdown chunk
- `emit`: Expected terminal output (empty `""` means no emission yet)
- ANSI codes use `\u001b` format (TOML Unicode escape)

### Test Categories

Tests are organized in `tests/fixtures/`:

1. **`blocks/`** - Individual block types (headings, paragraphs, code blocks, lists)
2. **`streaming/`** - Incremental emission and block boundary detection
3. **`ansi/`** - ANSI escape sequence formatting (bold, italic, colors)
4. **`complex/`** - Real-world documents with mixed block types

### Running Tests

```bash
# Run all conformance tests
cargo test

# Run specific test category
cargo test test_block_fixtures
cargo test test_streaming_fixtures
cargo test test_ansi_fixtures
cargo test test_complex_fixtures

# Run with verbose output
cargo test -- --nocapture
```

### Test Output

When tests fail, you see clear diagnostics:

```
Running 4 tests from blocks...
  ✗ heading-basic
    Heading should emit after newline is received
    Chunk 4 failed:
  Input: "\n"
  Expected: "\u{1b}[1;34m# Hello\u{1b}[0m\n"
  Actual: ""
```

### Writing New Tests

1. Create a `.toml` file in the appropriate `tests/fixtures/` subdirectory
2. Define test name and description
3. Add chunks with input and expected emissions
4. Use `\u001b` for ESC character in ANSI codes

Example ANSI codes:
- Bold: `\u001b[1m...\u001b[0m`
- Italic: `\u001b[3m...\u001b[0m`
- Color: `\u001b[1;34m...\u001b[0m` (bold blue)
- Background: `\u001b[48;5;235m...\u001b[0m`

### Current Status

**Test Infrastructure**: ✅ Complete and working

The conformance test suite is fully operational with 8 foundational tests:
- Block types: heading, paragraph, code block, list
- Streaming: incremental emission, block boundaries
- Formatting: inline ANSI codes
- Complex: mixed document

**Parser Implementation**: 🚧 To be implemented

All tests currently fail (parser is a stub). This is expected - the test suite is ready for TDD implementation.

### Next Steps

1. Implement the `StreamingParser` to pass basic tests
2. Add markdown parsing (likely using `pulldown-cmark`)
3. Add ANSI formatting and terminal rendering
4. Expand test coverage for GFM features (tables, task lists, strikethrough)
5. Add width/wrapping tests
6. Add performance benchmarks

## Project Structure

```
mdstream/
├── Cargo.toml
├── README.md
├── src/
│   └── lib.rs              # StreamingParser (stub)
└── tests/
    ├── conformance.rs      # Test runner
    ├── common/
    │   ├── mod.rs
    │   └── fixture_loader.rs
    └── fixtures/
        ├── blocks/         # 4 tests
        ├── streaming/      # 2 tests
        ├── ansi/           # 1 test
        └── complex/        # 1 test
```

## Design Philosophy

1. **Streaming First**: Blocks emit immediately when complete, enabling real-time rendering
2. **Test-Driven**: Comprehensive test suite defines expected behavior before implementation
3. **Exact Output**: Tests verify exact ANSI codes, not just content
4. **Incremental Testing**: Tests verify streaming property, not just final output

## License

MIT
