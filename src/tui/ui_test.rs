#[cfg(test)]
mod test_inline_code_spans {
    use super::*;
    use ratatui::style::{Color, Modifier, Style};

    #[test]
    fn test_parse_inline_code_to_spans_basic() {
        let text = "⟨INLINE_CODE⟩test⟨/INLINE_CODE⟩";
        let base_style = Style::default().fg(Color::White);
        let spans = parse_inline_code_to_spans(text, base_style);
        
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].content, "test");
        assert_eq!(spans[0].style.fg, Some(Color::Yellow));
    }

    #[test]
    fn test_parse_inline_code_to_spans_with_normal_text() {
        let text = "normal ⟨INLINE_CODE⟩code⟨/INLINE_CODE⟩ more";
        let base_style = Style::default().fg(Color::White);
        let spans = parse_inline_code_to_spans(text, base_style);
        
        assert_eq!(spans.len(), 3);
        assert_eq!(spans[0].content, "normal ");
        assert_eq!(spans[1].content, "code");
        assert_eq!(spans[2].content, " more");
    }
}
