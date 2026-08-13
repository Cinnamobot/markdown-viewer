use mdv::tui::themes::{default_config_path, UiTheme};
use mdv::tui::ThemeManager;
use ratatui::style::Color;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_parse_color_named() {
    assert_eq!(UiTheme::parse_color("red"), Color::Red);
    assert_eq!(UiTheme::parse_color("LightCyan"), Color::LightCyan);
    assert_eq!(UiTheme::parse_color("DARKGRAY"), Color::DarkGray);
    assert_eq!(UiTheme::parse_color("Grey"), Color::Gray);
}

#[test]
fn test_parse_color_hex_rgb() {
    assert_eq!(UiTheme::parse_color("#FF0000"), Color::Rgb(255, 0, 0));
    assert_eq!(UiTheme::parse_color("#00ff00"), Color::Rgb(0, 255, 0));
    assert_eq!(UiTheme::parse_color("#0000FF"), Color::Rgb(0, 0, 255));
    assert_eq!(UiTheme::parse_color("#AABBCC"), Color::Rgb(170, 187, 204));
}

#[test]
fn test_parse_color_hex_short() {
    assert_eq!(UiTheme::parse_color("#F00"), Color::Rgb(255, 0, 0));
    assert_eq!(UiTheme::parse_color("#abc"), Color::Rgb(170, 187, 204));
}

#[test]
fn test_parse_color_invalid() {
    assert_eq!(UiTheme::parse_color("not-a-color"), Color::Reset);
    assert_eq!(UiTheme::parse_color("#12345"), Color::Reset);
    assert_eq!(UiTheme::parse_color("#GGGGGG"), Color::Reset);
}

#[test]
fn test_default_config_path_windows() {
    std::env::set_var("HOME", "C:\\Users\\test");
    std::env::remove_var("XDG_CONFIG_HOME");
    std::env::remove_var("APPDATA");

    let path = default_config_path().unwrap();
    assert!(path.to_string_lossy().contains("mdv"));
    assert!(path.ends_with("theme.toml"));
}

#[test]
fn test_load_custom_theme_from_file() {
    let dir = tempdir().unwrap();
    let theme_path = dir.path().join("theme.toml");
    let toml_content = r##"
[heading]
h1 = "#FF0000"
h2 = "Cyan"
h3 = "Blue"
h4 = "Gray"
h5 = "DarkGray"
h6 = "DarkGray"

[code]
border = "#888888"
lang_label = "Magenta"

[list]
bullet = "Green"
checked = "Green"
unchecked = "Yellow"

[blockquote]
border = "Yellow"
text = "LightYellow"

[alert.note]
border = "Blue"
text = "LightBlue"
icon = "Blue"

[alert.tip]
border = "Green"
text = "LightGreen"
icon = "Green"

[alert.important]
border = "Magenta"
text = "LightMagenta"
icon = "Magenta"

[alert.warning]
border = "Yellow"
text = "LightYellow"
icon = "Yellow"

[alert.caution]
border = "Red"
text = "LightRed"
icon = "Red"

[table]
border = "Gray"
header = "Cyan"
cell = "White"

[inline_code]
foreground = "Black"
background = "LightGray"

[border]
primary = "Gray"
secondary = "LightGray"

[text]
primary = "White"
secondary = "Gray"
muted = "DarkGray"

[toc]
normal = "Gray"
selected = "White"
highlight_bg = "Blue"

[status_bar]
background = "Blue"
foreground = "White"
accent = "Cyan"

[layout]
wrap_text = true
toc_width_percent = 30
code_block_width_percent = 80
"##;
    std::fs::write(&theme_path, toml_content).unwrap();

    let mut manager = ThemeManager::new();
    manager
        .load_theme_from_file("custom".to_string(), &theme_path)
        .unwrap();

    assert!(manager.set_theme("custom"));
    let theme = manager.current_theme();
    assert_eq!(theme.heading.h1(), Color::Rgb(255, 0, 0));
    assert_eq!(theme.heading.h2(), Color::Cyan);
    assert!(theme.layout.wrap_text());
    assert_eq!(theme.layout.toc_width_percent(), 30);
    assert_eq!(theme.layout.code_block_width_percent(), 80);
}

#[test]
fn test_ui_theme_roundtrip_file() {
    let dir = tempdir().unwrap();
    let path: PathBuf = dir.path().join("theme.toml");

    let theme = UiTheme::dark();
    theme.save_to_file(&path).unwrap();

    let loaded = UiTheme::from_file(&path).unwrap();
    assert_eq!(loaded.heading.h1(), theme.heading.h1());
    assert_eq!(
        loaded.status_bar.background(),
        theme.status_bar.background()
    );
    assert_eq!(loaded.list.checked(), theme.list.checked());
}
