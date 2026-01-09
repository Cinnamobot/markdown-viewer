use mdv::markdown::{CodeHighlighter, MarkdownDocument};
use std::path::PathBuf;

#[test]
fn test_parse_basic_markdown() {
    let md = r#"# Title
## Subtitle
This is a paragraph.

- List item 1
- List item 2

```rust
fn main() {
    println!("Hello, world!");
}
```
"#;
    let highlighter = CodeHighlighter::new("base16-ocean.dark".to_string());
    let doc =
        MarkdownDocument::parse(PathBuf::from("test.md"), md.to_string(), &highlighter).unwrap();

    assert_eq!(doc.toc.len(), 2);
    assert_eq!(doc.toc[0].level, 1);
    assert_eq!(doc.toc[0].title, "Title");
    assert_eq!(doc.toc[1].level, 2);
    assert_eq!(doc.toc[1].title, "Subtitle");
}

#[test]
fn test_code_highlighting() {
    let md = r#"```rust
fn hello() {
    println!("test");
}
```
"#;
    let highlighter = CodeHighlighter::new("base16-ocean.dark".to_string());
    let doc =
        MarkdownDocument::parse(PathBuf::from("test.md"), md.to_string(), &highlighter).unwrap();

    // コードブロックが正しくパースされていることを確認
    assert!(!doc.parsed_lines.is_empty());
}

#[test]
fn test_empty_document() {
    let md = "";
    let highlighter = CodeHighlighter::new("base16-ocean.dark".to_string());
    let doc =
        MarkdownDocument::parse(PathBuf::from("test.md"), md.to_string(), &highlighter).unwrap();

    assert_eq!(doc.toc.len(), 0);
}
