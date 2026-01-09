use pulldown_cmark::{Event, Options, Parser};

#[test]
fn test_nested_list_events_debug() {
    let md = r#"
- Level 1 item 1
  - Level 2 item 1
  - Level 2 item 2
    - Level 3 item 1
- Level 1 item 2
"#;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(md, options);

    println!("\n=== Nested List Events ===");
    for (i, event) in parser.enumerate() {
        match &event {
            Event::Start(tag) => println!("{i}: Start({tag:?})"),
            Event::End(tag) => println!("{i}: End({tag:?})"),
            Event::Text(text) => println!("{i}: Text({text:?})"),
            Event::SoftBreak => println!("{i}: SoftBreak"),
            _ => println!("{i}: {event:?}"),
        }
    }
    println!("=== End Events ===\n");
}
