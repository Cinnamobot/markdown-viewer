use mdv::markdown::{CodeHighlighter, MarkdownDocument, ParsedLine};
use std::path::PathBuf;

#[test]
fn test_table_parsing() {
    let md = r#"
| Header 1 | Header 2 | Header 3 |
|----------|----------|----------|
| Cell 1   | Cell 2   | Cell 3   |
| Cell 4   | Cell 5   | Cell 6   |
"#;

    let highlighter = CodeHighlighter::new("base16-ocean.dark".to_string());
    let doc =
        MarkdownDocument::parse(PathBuf::from("test.md"), md.to_string(), &highlighter).unwrap();

    let tables: Vec<_> = doc
        .parsed_lines
        .iter()
        .filter_map(|line| {
            if let ParsedLine::Table { headers, rows, .. } = line {
                Some((headers, rows))
            } else {
                None
            }
        })
        .collect();

    assert_eq!(tables.len(), 1, "Should have 1 table");
    let (headers, rows) = tables[0];

    println!("Headers: {headers:?}");
    println!("Rows: {rows:?}");

    assert_eq!(headers.len(), 3, "Should have 3 headers");
    assert_eq!(headers[0].trim(), "Header 1");
    assert_eq!(headers[1].trim(), "Header 2");
    assert_eq!(headers[2].trim(), "Header 3");

    assert_eq!(rows.len(), 2, "Should have 2 rows");
    assert_eq!(rows[0].len(), 3, "First row should have 3 cells");
    assert_eq!(rows[1].len(), 3, "Second row should have 3 cells");
}

#[test]
fn test_table_alignment() {
    let md = r#"
| Left | Center | Right |
|:-----|:------:|------:|
| L    | C      | R     |
"#;

    let highlighter = CodeHighlighter::new("base16-ocean.dark".to_string());
    let doc =
        MarkdownDocument::parse(PathBuf::from("test.md"), md.to_string(), &highlighter).unwrap();

    let tables: Vec<_> = doc
        .parsed_lines
        .iter()
        .filter_map(|line| {
            if let ParsedLine::Table { alignments, .. } = line {
                Some(alignments)
            } else {
                None
            }
        })
        .collect();

    assert_eq!(tables.len(), 1);
    let alignments = tables[0];

    assert_eq!(alignments.len(), 3);
    assert_eq!(alignments[0], mdv::markdown::Alignment::Left);
    assert_eq!(alignments[1], mdv::markdown::Alignment::Center);
    assert_eq!(alignments[2], mdv::markdown::Alignment::Right);
}
