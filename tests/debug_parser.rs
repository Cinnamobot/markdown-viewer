use mdv::markdown::{CodeHighlighter, MarkdownDocument};
use std::path::PathBuf;

#[test]
fn test_debug_parsing() {
    let md = r#"# Main Title

This is a paragraph with some text.

## Section 1

Some more text here.

### Subsection

- Item 1
- Item 2
- Item 3

```rust
fn main() {
    println!("Hello!");
}
```

> This is a quote

---

More text after the rule.
"#;

    let highlighter = CodeHighlighter::new("base16-ocean.dark".to_string());
    let doc =
        MarkdownDocument::parse(PathBuf::from("test.md"), md.to_string(), &highlighter).unwrap();

    println!("Total parsed lines: {}", doc.parsed_lines.len());
    println!("TOC entries: {}", doc.toc.len());

    for (i, line) in doc.parsed_lines.iter().enumerate() {
        println!("{i}: {line:?}");
    }

    // 見出しの確認
    assert!(doc.toc.len() >= 3, "Expected at least 3 TOC entries");

    // パースされた行の種類を確認
    let has_heading = doc
        .parsed_lines
        .iter()
        .any(|l| matches!(l, mdv::markdown::ParsedLine::Heading { .. }));
    let has_code = doc
        .parsed_lines
        .iter()
        .any(|l| matches!(l, mdv::markdown::ParsedLine::Code { .. }));
    let has_list = doc
        .parsed_lines
        .iter()
        .any(|l| matches!(l, mdv::markdown::ParsedLine::ListItem { .. }));
    let has_quote = doc
        .parsed_lines
        .iter()
        .any(|l| matches!(l, mdv::markdown::ParsedLine::BlockQuote { .. }));

    assert!(has_heading, "Should have heading");
    assert!(has_code, "Should have code block");
    assert!(has_list, "Should have list");
    assert!(has_quote, "Should have blockquote");
}
