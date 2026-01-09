use once_cell::sync::Lazy;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, Theme, ThemeSet};
use syntect::parsing::SyntaxSet;

static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);
static THEME_SET: Lazy<ThemeSet> = Lazy::new(ThemeSet::load_defaults);

#[derive(Debug, Clone)]
pub struct StyledSpan {
    pub style: Style,
    pub text: String,
}

pub struct CodeHighlighter {
    theme_name: String,
}

impl CodeHighlighter {
    pub fn new(theme_name: String) -> Self {
        Self { theme_name }
    }

    fn get_theme(&self) -> &'static Theme {
        THEME_SET
            .themes
            .get(&self.theme_name)
            .or_else(|| THEME_SET.themes.get("base16-ocean.dark"))
            .expect("Default theme not found")
    }

    pub fn highlight(&self, code: &str, lang: Option<&str>) -> Vec<Vec<StyledSpan>> {
        let syntax = lang
            .and_then(|l| SYNTAX_SET.find_syntax_by_token(l))
            .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());

        let theme = self.get_theme();
        let mut highlighter = HighlightLines::new(syntax, theme);

        code.lines()
            .map(|line| {
                highlighter
                    .highlight_line(line, &SYNTAX_SET)
                    .unwrap_or_default()
                    .into_iter()
                    .map(|(style, text)| StyledSpan {
                        style,
                        text: text.to_string(),
                    })
                    .collect()
            })
            .collect()
    }
}
