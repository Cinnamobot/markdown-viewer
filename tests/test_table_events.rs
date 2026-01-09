use pulldown_cmark::{Event, Options, Parser};

#[test]
fn test_table_events_debug() {
    let md = r#"
| Header 1 | Header 2 | Header 3 |
|----------|----------|----------|
| Cell 1   | Cell 2   | Cell 3   |
"#;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(md, options);

    println!("\n=== Table Events ===");
    for (i, event) in parser.enumerate() {
        match &event {
            Event::Start(tag) => println!("{i}: Start({tag:?})"),
            Event::End(tag) => println!("{i}: End({tag:?})"),
            Event::Text(text) => println!("{i}: Text({text:?})"),
            _ => println!("{i}: {event:?}"),
        }
    }
    println!("=== End Events ===\n");
}
