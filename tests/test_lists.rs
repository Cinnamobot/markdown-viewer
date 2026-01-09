use mdv::markdown::{CodeHighlighter, MarkdownDocument, ParsedLine};
use std::path::PathBuf;

#[test]
fn test_checkbox_list() {
    let md = r#"
- [x] Completed task
- [ ] Incomplete task
- [X] Another completed
- Normal item
"#;

    let highlighter = CodeHighlighter::new("base16-ocean.dark".to_string());
    let doc =
        MarkdownDocument::parse(PathBuf::from("test.md"), md.to_string(), &highlighter).unwrap();

    let list_items: Vec<_> = doc
        .parsed_lines
        .iter()
        .filter_map(|line| {
            if let ParsedLine::ListItem {
                checked, content, ..
            } = line
            {
                Some((checked, content))
            } else {
                None
            }
        })
        .collect();

    assert_eq!(list_items.len(), 4);
    assert_eq!(list_items[0].0, &Some(true)); // [x]
    assert_eq!(list_items[1].0, &Some(false)); // [ ]
    assert_eq!(list_items[2].0, &Some(true)); // [X]
    assert_eq!(list_items[3].0, &None); // normal
}

#[test]
fn test_nested_list() {
    let md = r#"
- Level 1 item 1
  - Level 2 item 1
  - Level 2 item 2
    - Level 3 item 1
- Level 1 item 2
"#;

    let highlighter = CodeHighlighter::new("base16-ocean.dark".to_string());
    let doc =
        MarkdownDocument::parse(PathBuf::from("test.md"), md.to_string(), &highlighter).unwrap();

    let list_items: Vec<_> = doc
        .parsed_lines
        .iter()
        .filter_map(|line| {
            if let ParsedLine::ListItem {
                indent, content, ..
            } = line
            {
                Some((*indent, content.clone()))
            } else {
                None
            }
        })
        .collect();

    println!("List items: {list_items:?}");

    assert!(list_items.len() >= 4, "Should have at least 4 list items");

    // いくつかのアイテムはネストされているはず
    let has_nested = list_items.iter().any(|(indent, _)| *indent > 0);
    assert!(has_nested, "Should have nested items");
}
