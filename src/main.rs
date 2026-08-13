use clap::Parser;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use mdv::{
    cli::Cli,
    error::MdError,
    markdown::{CodeHighlighter, MarkdownDocument},
    tui::{self, App, ThemeManager},
    watcher::{LiveReloader, ReloadEvent},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

#[tokio::main]
async fn main() -> Result<(), MdError> {
    let cli = Cli::parse();

    let content = std::fs::read_to_string(&cli.path)?;

    let highlighter = CodeHighlighter::new(cli.theme.clone());

    let document = MarkdownDocument::parse(cli.path.clone(), content, &highlighter)?;

    let mut theme_manager = ThemeManager::new();

    // カスタムテーマ設定を読み込む
    // 優先順位: --config > デフォルト設定パス > --ui-theme
    let config_loaded = if let Some(config_path) = &cli.config {
        theme_manager
            .load_theme_from_file("custom".to_string(), config_path)
            .map_err(|e| MdError::ThemeLoadError(format!("{}: {e}", config_path.display())))?;
        true
    } else if let Some(default_path) = tui::themes::default_config_path() {
        if default_path.exists() {
            match theme_manager.load_theme_from_file("custom".to_string(), &default_path) {
                Ok(()) => true,
                Err(e) => {
                    eprintln!(
                        "Failed to load custom theme from {}: {e}",
                        default_path.display()
                    );
                    false
                }
            }
        } else {
            false
        }
    } else {
        false
    };

    if config_loaded {
        theme_manager.set_theme("custom");
    } else {
        theme_manager.set_theme(&cli.ui_theme);
    }

    let mut app = App::new(document, cli.show_toc, cli.line, &theme_manager);

    if let Some(heading) = &cli.heading {
        app.jump_to_heading_by_name(heading);
    }

    let mut watcher = if !cli.no_watch {
        Some(LiveReloader::new(cli.path.clone())?)
    } else {
        None
    };

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(
        &mut terminal,
        &mut app,
        &mut watcher,
        &highlighter,
        &theme_manager,
        &cli,
    )
    .await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

async fn run_app<'a>(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App<'a>,
    watcher: &mut Option<LiveReloader>,
    highlighter: &CodeHighlighter,
    theme_manager: &'a ThemeManager,
    cli: &Cli,
) -> Result<(), MdError> {
    let mut event_handler = tui::events::EventHandler::new();

    loop {
        terminal.draw(|f| tui::ui::render(f, app, theme_manager))?;

        tokio::select! {
            key_event = event_handler.next_key() => {
                if let Some(key) = key_event? {
                    app.handle_key(key.code, key.modifiers);
                    if app.should_quit {
                        break;
                    }
                    // チェックボックスをトグルした場合は再パースして表示を更新
                    if let Some(new_content) = app.take_updated_content() {
                        match MarkdownDocument::parse(cli.path.clone(), new_content, highlighter) {
                            Ok(new_document) => {
                                app.update_document(new_document);
                            }
                            Err(e) => {
                                eprintln!("Failed to parse markdown: {e}");
                            }
                        }
                    }
                }
            }
            reload_event = async {
                match watcher {
                    Some(w) => w.next_event().await,
                    None => None,
                }
            } => {
                if let Some(event) = reload_event {
                    match event {
                        ReloadEvent::FileChanged(_) => {
                            match std::fs::read_to_string(&cli.path) {
                                Ok(content) => {
                                    match MarkdownDocument::parse(
                                        cli.path.clone(),
                                        content,
                                        highlighter,
                                    ) {
                                        Ok(new_document) => {
                                            app.update_document(new_document);
                                        }
                                        Err(e) => {
                                            eprintln!("Failed to parse markdown: {e}");
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Failed to read file: {e}");
                                }
                            }
                        }
                        ReloadEvent::Error(err) => {
                            eprintln!("File watcher error: {err}");
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
