use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "mdv",
    version,
    about = "Ultra-lightweight markdown viewer for terminal",
    long_about = None
)]
pub struct Cli {
    /// Path to the markdown file
    #[arg(value_name = "FILE")]
    pub path: PathBuf,

    /// Disable live reload
    #[arg(short = 'n', long)]
    pub no_watch: bool,

    /// Syntax highlighting theme (default: base16-eighties.dark)
    #[arg(short = 't', long, default_value = "base16-eighties.dark")]
    pub theme: String,

    /// Start with table of contents open
    #[arg(long)]
    pub show_toc: bool,

    /// Jump to specific line number
    #[arg(short = 'l', long)]
    pub line: Option<usize>,

    /// Jump to heading (fuzzy search)
    #[arg(short = 'H', long)]
    pub heading: Option<String>,
}
