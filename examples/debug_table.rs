use mdv::markdown::{highlighter::CodeHighlighter, parser::MarkdownDocument, ParsedLine};
use std::path::PathBuf;

fn main() {
    let content = std::fs::read_to_string("examples/sample.md").unwrap();
    let highlighter = CodeHighlighter::new("base16-ocean.dark".to_string());
    let doc = MarkdownDocument::parse(
        PathBuf::from("examples/sample.md"),
        content.clone(),
        &highlighter,
    )
    .unwrap();

    // Find the keybindings table
    for line in &doc.parsed_lines {
        if let ParsedLine::Table { headers, rows, .. } = line {
            if headers.len() == 2 && headers[0] == "Key" {
                println!("Keybindings Table:");
                println!("Headers: {:?}", headers);
                for (i, row) in rows.iter().enumerate() {
                    println!("Row {}: {:?}", i, row);
                }
            }
        }
    }
}
