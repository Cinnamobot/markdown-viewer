# mdv - Ultra-Lightweight Markdown Viewer

Welcome to **mdv**, a lightning-fast markdown viewer for the terminal!

## Features

- **Lightning-fast startup** - Opens instantly, even large files
- **Live reload** - Automatically updates when the file changes
- **Syntax highlighting** - Beautiful code blocks with color
- **Table of contents** - Quick navigation with `t` key
- **Low memory footprint** - Uses less than 10MB of RAM
- **SSH-friendly** - Works perfectly over remote connections

## Installation

Install mdv using cargo:

```bash
cargo install mdv
```

## Usage

Basic usage:

```bash
mdv README.md
```

With options:

```bash
# Disable live reload
mdv -n README.md

# Start with table of contents open
mdv --show-toc document.md

# Jump to specific line
mdv -l 100 README.md

# Jump to heading
mdv -H "Installation" README.md
```

## Keybindings

| Key | Action |
|-----|--------|
| `j` / `↓` | Scroll down |
| `k` / `↑` | Scroll up |
| `g` | Jump to top |
| `G` | Jump to bottom |
| `PageDown/PageUp` | Page scroll |
| `t` | Toggle table of contents |
| `Enter` | Jump to selected heading (in TOC) |
| `?` | Show help |
| `q` / `Ctrl+C` | Quit |

## GitHub Alerts

> [!NOTE]
> Useful information that users should know.

> [!TIP]
> A helpful tip for a better experience.

> [!IMPORTANT]
> Key information users need to know.

> [!WARNING]
> Urgent info that needs immediate user attention.

> [!CAUTION]
> Advises about certain risks.

## Code Example

Here's a simple Rust example:

```rust
fn main() {
    println!("Hello, mdv!");

    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();

    println!("Sum: {}", sum);
}
```

And a Python example:

```python
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)

# Print first 10 fibonacci numbers
for i in range(10):
    print(f"fib({i}) = {fibonacci(i)}")
```

And a JavaScript example:

```javascript
const greet = (name) => {
  const message = `Hello, ${name}!`;
  console.log(message);
  return message;
};

// Async example
async function fetchData(url) {
  const res = await fetch(url);
  return res.json();
}
```

Inline code like `let x = 42;` or `cargo build --release` is rendered with a distinctive style.

## Lists

Unordered list:
- First item
- Second item
- Third item

Nested list:
- Parent item
  - Child item 1
  - Child item 2
    - Grandchild item
  - Child item 3
- Another parent item

Task list (checkboxes):
- [x] Completed task
- [ ] Incomplete task
- [x] Another completed task
- [ ] Todo: Add more features

Nested task list:
- [x] Release v0.2.0
- [ ] Prepare for U22 contest
  - [ ] Polish README
  - [ ] Record demo video
  - [x] Fix encoding issues

## Blockquotes

> This is a blockquote.
> It can span multiple lines.
>
> Perfect for highlighting important information!

## Horizontal Rule

---

## Tables

Basic table:

| Feature | Support | Notes |
|---------|---------|-------|
| Headers | ✓ | Bold and colored |
| Borders | ✓ | Box drawing characters |
| Alignment | ✓ | Left, center, right |
| Colors | ✓ | Syntax highlighting |

Table with alignment:

| Left aligned | Center aligned | Right aligned |
|:-------------|:--------------:|--------------:|
| Left | Center | Right |
| Data | More data | Numbers |
| Text | Content | 123 |

## Why mdv?

Traditional markdown viewers have drawbacks:
- **Browsers** consume 100-500MB of memory
- **IDEs** are heavy and slow to start
- **cat/less** don't render markdown properly

**mdv** solves all these problems:
- **Memory efficient**: < 10MB RAM usage
- **Fast startup**: < 50ms
- **Beautiful rendering**: Full markdown support
- **Live updates**: See changes instantly

## License

MIT License - feel free to use and modify!
