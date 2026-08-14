use mdv::tui::{ThemeManager, UiTheme};
use ratatui::style::Color;

#[test]
fn test_parse_color_named() {
    assert_eq!(UiTheme::parse_color("red"), Color::Red);
    assert_eq!(UiTheme::parse_color("LightCyan"), Color::LightCyan);
    assert_eq!(UiTheme::parse_color("DARKGRAY"), Color::DarkGray);
    assert_eq!(UiTheme::parse_color("Grey"), Color::Gray);
    assert_eq!(UiTheme::parse_color("white"), Color::White);
}

#[test]
fn test_parse_color_unknown_falls_back_to_white() {
    assert_eq!(UiTheme::parse_color("not-a-color"), Color::White);
}

#[test]
fn test_default_theme_is_dark() {
    let manager = ThemeManager::new();
    assert_eq!(manager.current_theme_name(), "dark");
}

#[test]
fn test_theme_switching_dark_light() {
    let mut manager = ThemeManager::new();

    assert!(manager.set_theme("light"));
    assert_eq!(manager.current_theme_name(), "light");
    let light = manager.current_theme();
    assert_eq!(light.heading.h1(), Color::Blue);

    assert!(manager.set_theme("dark"));
    let dark = manager.current_theme();
    assert_eq!(dark.heading.h1(), Color::Cyan);
    assert_ne!(dark.heading.h1(), light.heading.h1());
}

#[test]
fn test_unknown_theme_is_rejected() {
    let mut manager = ThemeManager::new();
    assert!(!manager.set_theme("nonexistent"));
    assert_eq!(manager.current_theme_name(), "dark");
}

#[test]
fn test_available_themes() {
    let manager = ThemeManager::new();
    let themes = manager.available_themes();
    assert!(themes.iter().any(|t| t.as_str() == "dark"));
    assert!(themes.iter().any(|t| t.as_str() == "light"));
}
