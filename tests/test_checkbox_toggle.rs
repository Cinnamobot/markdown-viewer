use mdv::markdown::parser::toggle_checkbox_in_content;
use mdv::markdown::{CodeHighlighter, MarkdownDocument, ParsedLine};
use mdv::tui::{App, ThemeManager};
use std::path::PathBuf;
use tempfile::tempdir;

fn parse(md: &str) -> MarkdownDocument {
    let highlighter = CodeHighlighter::new("base16-ocean.dark".to_string());
    MarkdownDocument::parse(PathBuf::from("test.md"), md.to_string(), &highlighter).unwrap()
}

#[test]
fn test_checkbox_line_numbers() {
    let md = "# Title\n\n- [x] Completed task\n- [ ] Incomplete task\n- Normal item\n\n## Section\n\n- [ ] Nested list item\n";
    let doc = parse(md);

    let task_lines: Vec<(bool, usize)> = doc
        .parsed_lines
        .iter()
        .filter_map(|line| {
            if let ParsedLine::ListItem {
                checked: Some(checked),
                line_num: Some(line_num),
                ..
            } = line
            {
                Some((*checked, *line_num))
            } else {
                None
            }
        })
        .collect();

    assert_eq!(task_lines.len(), 3);
    assert_eq!(task_lines[0], (true, 2)); // 0始まり: "- [x] Completed task" は3行目
    assert_eq!(task_lines[1], (false, 3));
    assert_eq!(task_lines[2], (false, 8));

    // 通常リストにはline_numが無い
    let normal_items = doc
        .parsed_lines
        .iter()
        .filter_map(|line| {
            if let ParsedLine::ListItem {
                checked: None,
                line_num,
                ..
            } = line
            {
                Some(*line_num)
            } else {
                None
            }
        })
        .count();
    assert_eq!(normal_items, 1);
}

#[test]
fn test_toggle_unchecked_to_checked() {
    let content = "# Tasks\n\n- [ ] Write code\n- [x] Done task\n";
    let toggled = toggle_checkbox_in_content(content, 2).unwrap();

    assert_eq!(toggled, "# Tasks\n\n- [x] Write code\n- [x] Done task\n");
}

#[test]
fn test_toggle_checked_to_unchecked() {
    let content = "- [x] Done task\n- [ ] Pending\n";
    let toggled = toggle_checkbox_in_content(content, 0).unwrap();

    assert_eq!(toggled, "- [ ] Done task\n- [ ] Pending\n");
}

#[test]
fn test_toggle_uppercase_x() {
    let content = "- [X] Done with caps\n";
    let toggled = toggle_checkbox_in_content(content, 0).unwrap();
    assert_eq!(toggled, "- [ ] Done with caps\n");
}

#[test]
fn test_toggle_no_marker_returns_none() {
    let content = "- Normal item\n- [ ] Task\n";
    assert!(toggle_checkbox_in_content(content, 0).is_none());
}

#[test]
fn test_toggle_out_of_range_returns_none() {
    let content = "- [ ] Task\n";
    assert!(toggle_checkbox_in_content(content, 10).is_none());
}

#[test]
fn test_toggle_preserves_crlf() {
    let content = "- [ ] Task\r\n- [x] Done\r\n";
    let toggled = toggle_checkbox_in_content(content, 0).unwrap();
    assert_eq!(toggled, "- [x] Task\r\n- [x] Done\r\n");
}

#[test]
fn test_toggle_indented_nested_task() {
    let content = "- Parent\n  - [ ] Child task\n  - [x] Done child\n";
    let toggled = toggle_checkbox_in_content(content, 1).unwrap();
    assert_eq!(
        toggled,
        "- Parent\n  - [x] Child task\n  - [x] Done child\n"
    );
}

#[test]
fn test_app_toggle_writes_file_and_marks_update() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("tasks.md");
    std::fs::write(&file_path, "- [ ] Task A\n- [x] Task B\n").unwrap();

    let highlighter = CodeHighlighter::new("base16-ocean.dark".to_string());
    let content = std::fs::read_to_string(&file_path).unwrap();
    let document = MarkdownDocument::parse(file_path.clone(), content, &highlighter).unwrap();

    let theme_manager = ThemeManager::new();
    let mut app = App::new(document, false, None, &theme_manager);

    // カーソルを0行目（未チェックのタスク）に合わせてトグル
    assert!(app.toggle_current_checkbox().is_ok());

    // ファイルに書き戻されたことを確認
    let written = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(written, "- [x] Task A\n- [x] Task B\n");

    // 再パース用の新しいコンテンツが保持されている
    let updated = app.take_updated_content().unwrap();
    assert!(updated.contains("- [x] Task A"));

    // 再パースするとcheckedが反映される
    let reloaded = parse(&updated);
    let first = &reloaded.parsed_lines[0];
    assert!(matches!(
        first,
        ParsedLine::ListItem {
            checked: Some(true),
            ..
        }
    ));
}
#[test]
fn test_app_toggle_non_task_line_is_noop() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("doc.md");
    std::fs::write(&file_path, "# Heading\n\nJust text\n").unwrap();

    let highlighter = CodeHighlighter::new("base16-ocean.dark".to_string());
    let content = std::fs::read_to_string(&file_path).unwrap();
    let document = MarkdownDocument::parse(file_path.clone(), content, &highlighter).unwrap();

    let theme_manager = ThemeManager::new();
    let mut app = App::new(document, false, None, &theme_manager);

    assert!(app.toggle_current_checkbox().is_ok());
    assert!(app.take_updated_content().is_none());
    assert_eq!(
        std::fs::read_to_string(&file_path).unwrap(),
        "# Heading\n\nJust text\n"
    );
}

#[test]
fn test_space_key_toggles_checkbox() {
    use crossterm::event::{KeyCode, KeyModifiers};

    let dir = tempdir().unwrap();
    let file_path = dir.path().join("tasks.md");
    std::fs::write(&file_path, "- [ ] Task A\n- [x] Task B\n").unwrap();

    let highlighter = CodeHighlighter::new("base16-ocean.dark".to_string());
    let content = std::fs::read_to_string(&file_path).unwrap();
    let document = MarkdownDocument::parse(file_path.clone(), content, &highlighter).unwrap();

    let theme_manager = ThemeManager::new();
    let mut app = App::new(document, false, None, &theme_manager);

    app.handle_key(KeyCode::Char(' '), KeyModifiers::NONE);

    let updated = app.take_updated_content().unwrap();
    assert!(updated.contains("- [x] Task A"));
    assert!(app.status_message.is_some());
}

#[test]
fn test_help_toggle_via_key() {
    use crossterm::event::{KeyCode, KeyModifiers};

    let highlighter = CodeHighlighter::new("base16-ocean.dark".to_string());
    let document = MarkdownDocument::parse(
        PathBuf::from("test.md"),
        "# Title\n".to_string(),
        &highlighter,
    )
    .unwrap();
    let theme_manager = ThemeManager::new();
    let mut app = App::new(document, false, None, &theme_manager);

    assert!(!app.show_help);
    app.handle_key(KeyCode::Char('?'), KeyModifiers::NONE);
    assert!(app.show_help);

    // ヘルプ表示中は他のキーで閉じない
    app.handle_key(KeyCode::Char('j'), KeyModifiers::NONE);
    assert!(app.show_help);

    // Esc で閉じる
    app.handle_key(KeyCode::Esc, KeyModifiers::NONE);
    assert!(!app.show_help);

    // q でも閉じる
    app.handle_key(KeyCode::Char('?'), KeyModifiers::NONE);
    assert!(app.show_help);
    app.handle_key(KeyCode::Char('q'), KeyModifiers::NONE);
    assert!(!app.show_help);
}
