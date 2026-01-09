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

    println!("Parsed list items around line 94-100:");
    println!("=====================================\n");
    
    let mut found_nested_list = false;
    for line in &doc.parsed_lines {
        if let ParsedLine::ListItem {
            indent,
            content,
            checked,
        } = line
        {
            if content.contains("Parent") || content.contains("Child") || content.contains("Another parent") || content.contains("Grandchild") {
                found_nested_list = true;
                let indent_str = "  ".repeat(*indent);
                let marker = if checked.is_some() { "[ ]" } else { "●" };
                println!("{}{} {}", indent_str, marker, content);
            } else if found_nested_list && !content.contains("Parent") && !content.contains("Child") && !content.contains("Grandchild") {
                // Stop after we've passed the nested list section
                if content.contains("Completed") {
                    break;
                }
            }
        }
    }
}
