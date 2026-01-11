use clap::Parser;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use mdv::{
    cli::Cli,
    markdown::{CodeHighlighter, MarkdownDocument},
    tui::{self, App},
    watcher::{LiveReloader, ReloadEvent},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // ファイル読み込み
    let content = std::fs::read_to_string(&cli.path)?;

    // ハイライター初期化
    let highlighter = CodeHighlighter::new(cli.theme.clone());

    // マークダウンパース
    let document = MarkdownDocument::parse(cli.path.clone(), content, &highlighter)?;

    // アプリケーション初期化
    let mut app = App::new(document, cli.show_toc, cli.line);

    // 見出しへのジャンプ
    if let Some(heading) = &cli.heading {
        app.jump_to_heading_by_name(heading);
    }

    // ファイル監視開始
    let mut watcher = if !cli.no_watch {
        Some(LiveReloader::new(cli.path.clone())?)
    } else {
        None
    };

    // ターミナル初期化
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // メインループ
    let result = run_app(&mut terminal, &mut app, &mut watcher, &highlighter, &cli).await;

    // ターミナル復元
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    watcher: &mut Option<LiveReloader>,
    highlighter: &CodeHighlighter,
    cli: &Cli,
) -> anyhow::Result<()> {
    let mut event_handler = tui::events::EventHandler::new();

    loop {
        terminal.draw(|f| tui::ui::render(f, app))?;

        // イベント処理（真の非同期処理）
        tokio::select! {
            key_event = event_handler.next_key() => {
                if let Some(key) = key_event? {
                    app.handle_key(key.code, key.modifiers);
                    if app.should_quit {
                        break;
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
                                            eprintln!("Failed to parse markdown: {}", e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Failed to read file: {}", e);
                                }
                            }
                        }
                        ReloadEvent::Error(err) => {
                            eprintln!("File watcher error: {}", err);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
