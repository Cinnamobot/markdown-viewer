use pulldown_cmark::{Event, Options, Parser};

#[test]
fn test_checkbox_events_debug() {
    let md = r#"
- [x] Completed task
- [ ] Incomplete task
"#;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(md, options);

    println!("\n=== Checkbox Events ===");
    for (i, event) in parser.enumerate() {
        match &event {
            Event::Start(tag) => println!("{i}: Start({tag:?})"),
            Event::End(tag) => println!("{i}: End({tag:?})"),
            Event::Text(text) => println!("{i}: Text({text:?})"),
            Event::TaskListMarker(checked) => {
                println!("{i}: TaskListMarker(checked={checked})")
            }
            _ => println!("{i}: {event:?}"),
        }
    }
    println!("=== End Events ===\n");
}
