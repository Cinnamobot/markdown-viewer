use mdv::markdown::{CodeHighlighter, MarkdownDocument, ParsedLine};
use std::path::PathBuf;

#[test]
fn test_list_indent_detailed() {
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

    println!("\nList items found:");
    for (i, (indent, content)) in list_items.iter().enumerate() {
        println!("  {i}: indent={indent}, content={content:?}");
    }

    // Level 1のアイテムはインデント0であるべき
    // Level 2のアイテムはインデント1であるべき
    // Level 3のアイテムはインデント2であるべき

    assert!(
        list_items.len() >= 5,
        "Should have at least 5 list items, got {}",
        list_items.len()
    );

    // 各レベルの確認
    let level1_items: Vec<_> = list_items
        .iter()
        .filter(|(indent, _)| *indent == 0)
        .collect();
    let level2_items: Vec<_> = list_items
        .iter()
        .filter(|(indent, _)| *indent == 1)
        .collect();
    let level3_items: Vec<_> = list_items
        .iter()
        .filter(|(indent, _)| *indent == 2)
        .collect();

    println!("\nLevel 1 items (indent=0): {level1_items:?}");
    println!("Level 2 items (indent=1): {level2_items:?}");
    println!("Level 3 items (indent=2): {level3_items:?}");

    assert_eq!(level1_items.len(), 2, "Should have 2 level-1 items");
    assert_eq!(level2_items.len(), 2, "Should have 2 level-2 items");
    assert_eq!(level3_items.len(), 1, "Should have 1 level-3 item");
}
