#[cfg(test)]
mod tests {
    use crate::markdown::{highlighter::CodeHighlighter, parser::MarkdownDocument, ParsedLine};
    use std::path::PathBuf;

    #[test]
    fn test_nested_list_parent_item_appears_first() {
        let markdown = r#"Nested list:
- Parent item
  - Child item 1
  - Child item 2
    - Grandchild item
  - Child item 3
- Another parent item"#;

        let highlighter = CodeHighlighter::new("base16-ocean.dark".to_string());
        let doc = MarkdownDocument::parse(
            PathBuf::from("test.md"),
            markdown.to_string(),
            &highlighter,
        )
        .unwrap();

        // Extract list items
        let list_items: Vec<(&usize, &String)> = doc
            .parsed_lines
            .iter()
            .filter_map(|line| {
                if let ParsedLine::ListItem {
                    indent, content, ..
                } = line
                {
                    Some((indent, content))
                } else {
                    None
                }
            })
            .collect();

        // Verify we have all items
        assert_eq!(list_items.len(), 6);

        // Verify order: Parent item should appear BEFORE its children
        assert_eq!(list_items[0].1, "Parent item");
        assert_eq!(*list_items[0].0, 0); // indent 0

        assert_eq!(list_items[1].1, "Child item 1");
        assert_eq!(*list_items[1].0, 1); // indent 1

        assert_eq!(list_items[2].1, "Child item 2");
        assert_eq!(*list_items[2].0, 1); // indent 1

        assert_eq!(list_items[3].1, "Grandchild item");
        assert_eq!(*list_items[3].0, 2); // indent 2

        assert_eq!(list_items[4].1, "Child item 3");
        assert_eq!(*list_items[4].0, 1); // indent 1

        assert_eq!(list_items[5].1, "Another parent item");
        assert_eq!(*list_items[5].0, 0); // indent 0
    }

    #[test]
    fn test_inline_code_parsing() {
        let markdown = "Text with `inline code` here";
        let highlighter = CodeHighlighter::new("base16-ocean.dark".to_string());
        let doc = MarkdownDocument::parse(
            PathBuf::from("test.md"),
            markdown.to_string(),
            &highlighter,
        )
        .unwrap();

        // Find the text line
        let text_line = doc
            .parsed_lines
            .iter()
            .find_map(|line| {
                if let ParsedLine::Text { content } = line {
                    Some(content)
                } else {
                    None
                }
            })
            .expect("Should have text line");

        // Should contain markers, not backticks
        assert!(
            text_line.contains("⟨INLINE_CODE⟩"),
            "Should contain opening marker"
        );
        assert!(
            text_line.contains("⟨/INLINE_CODE⟩"),
            "Should contain closing marker"
        );
        assert!(
            text_line.contains("inline code"),
            "Should contain the code content"
        );

        // Should NOT have standalone backticks anymore
        // Note: the markers contain the content between them, backticks should not be separate
        let expected = "Text with ⟨INLINE_CODE⟩inline code⟨/INLINE_CODE⟩ here";
        assert_eq!(text_line, expected);
    }

    #[test]
    fn test_inline_code_in_table() {
        let markdown = r#"| Key | Action |
|-----|--------|
| `j` | Scroll down |
| `k` | Scroll up |"#;

        let highlighter = CodeHighlighter::new("base16-ocean.dark".to_string());
        let doc = MarkdownDocument::parse(
            PathBuf::from("test.md"),
            markdown.to_string(),
            &highlighter,
        )
        .unwrap();

        // Find the table
        let table = doc
            .parsed_lines
            .iter()
            .find_map(|line| {
                if let ParsedLine::Table {
                    headers,
                    rows,
                    alignments,
                } = line
                {
                    Some((headers, rows, alignments))
                } else {
                    None
                }
            })
            .expect("Should have table");

        // Check that table cells contain markers
        let (headers, rows, _) = table;
        assert_eq!(headers.len(), 2);
        assert_eq!(rows.len(), 2);

        // First row should have inline code markers
        assert!(
            rows[0][0].contains("⟨INLINE_CODE⟩"),
            "First cell should contain inline code marker"
        );
        assert_eq!(rows[0][0], "⟨INLINE_CODE⟩j⟨/INLINE_CODE⟩");
        
        // Second row should also have inline code markers
        assert!(
            rows[1][0].contains("⟨INLINE_CODE⟩"),
            "Second cell should contain inline code marker"
        );
        assert_eq!(rows[1][0], "⟨INLINE_CODE⟩k⟨/INLINE_CODE⟩");
    }

    #[test]
    fn test_truncate_with_markers() {
        use crate::tui::ui::{truncate_with_markers, visible_text_len};
        
        let text = "⟨INLINE_CODE⟩PageDown/PageUp⟨/INLINE_CODE⟩";
        // Width 15 is exact length of visible text
        let truncated = truncate_with_markers(text, 15);
        assert_eq!(truncated, "⟨INLINE_CODE⟩PageDown/PageUp⟨/INLINE_CODE⟩");

        // Width 10 cuts it short -> should still have closing marker
        let truncated_short = truncate_with_markers(text, 10);
        assert_eq!(truncated_short, "⟨INLINE_CODE⟩PageDown/P⟨/INLINE_CODE⟩");
        
        // Check normal text mix
        let mixed = "Start ⟨INLINE_CODE⟩code⟨/INLINE_CODE⟩ end";
        let mixed_trunc = truncate_with_markers(mixed, 8); // "Start co"
        assert_eq!(mixed_trunc, "Start ⟨INLINE_CODE⟩co⟨/INLINE_CODE⟩");

        // Test CJK characters
        let cjk_text = "日本語テスト";
        assert_eq!(visible_text_len(cjk_text), 12); // 6 chars * 2 width = 12

        // Truncate CJK (width 12 -> 8) "日本語テ"
        let cjk_trunc = truncate_with_markers(cjk_text, 8);
        assert_eq!(cjk_trunc, "日本語テ");

        // Truncate CJK odd width (width 12 -> 7) 
        // "日本語" is width 6. "日本語テ" is width 8.
        // Width 7 should allow "日本語" (6) but not "テ" (adds 2 -> 8).
        let cjk_odd = truncate_with_markers(cjk_text, 7);
        assert_eq!(cjk_odd, "日本語");
        
        // Test ambiguous width characters (arrows)
        // With width() (not cjk), arrows should be width 1
        let arrow_text = "j / ↓";
        // j(1) + space(1) + /(1) + space(1) + ↓(1) = 5
        assert_eq!(visible_text_len(arrow_text), 5);
        
        let arrow_trunc = truncate_with_markers(arrow_text, 4);
        assert_eq!(arrow_trunc, "j / ");
    }
}
