use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// 見出しの色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingColors {
    pub h1: String,
    pub h2: String,
    pub h3: String,
    pub h4: String,
    pub h5: String,
    pub h6: String,
}

impl HeadingColors {
    pub fn h1(&self) -> Color {
        UiTheme::parse_color(&self.h1)
    }
    pub fn h2(&self) -> Color {
        UiTheme::parse_color(&self.h2)
    }
    pub fn h3(&self) -> Color {
        UiTheme::parse_color(&self.h3)
    }
    pub fn h4(&self) -> Color {
        UiTheme::parse_color(&self.h4)
    }
    pub fn h5(&self) -> Color {
        UiTheme::parse_color(&self.h5)
    }
    pub fn h6(&self) -> Color {
        UiTheme::parse_color(&self.h6)
    }
}

/// コードブロックの色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeColors {
    pub border: String,
    pub lang_label: String,
}

impl CodeColors {
    pub fn border(&self) -> Color {
        UiTheme::parse_color(&self.border)
    }
    pub fn lang_label(&self) -> Color {
        UiTheme::parse_color(&self.lang_label)
    }
}

/// リストの色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListColors {
    pub bullet: String,
    pub checked: String,
    pub unchecked: String,
}

impl ListColors {
    pub fn bullet(&self) -> Color {
        UiTheme::parse_color(&self.bullet)
    }
    pub fn checked(&self) -> Color {
        UiTheme::parse_color(&self.checked)
    }
    pub fn unchecked(&self) -> Color {
        UiTheme::parse_color(&self.unchecked)
    }
}

/// 引用の色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockquoteColors {
    pub border: String,
    pub text: String,
}

impl BlockquoteColors {
    pub fn border(&self) -> Color {
        UiTheme::parse_color(&self.border)
    }
    pub fn text(&self) -> Color {
        UiTheme::parse_color(&self.text)
    }
}

/// アラートタイプの色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertTypeColors {
    pub border: String,
    pub text: String,
    pub icon: String,
}

impl AlertTypeColors {
    pub fn border(&self) -> Color {
        UiTheme::parse_color(&self.border)
    }
    pub fn text(&self) -> Color {
        UiTheme::parse_color(&self.text)
    }
    pub fn icon(&self) -> Color {
        UiTheme::parse_color(&self.icon)
    }
}

/// アラートの色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertColors {
    pub note: AlertTypeColors,
    pub tip: AlertTypeColors,
    pub important: AlertTypeColors,
    pub warning: AlertTypeColors,
    pub caution: AlertTypeColors,
}

/// テーブルの色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableColors {
    pub border: String,
    pub header: String,
    pub cell: String,
}

impl TableColors {
    pub fn border(&self) -> Color {
        UiTheme::parse_color(&self.border)
    }
    pub fn header(&self) -> Color {
        UiTheme::parse_color(&self.header)
    }
    pub fn cell(&self) -> Color {
        UiTheme::parse_color(&self.cell)
    }
}

/// インラインコードの色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineCodeColors {
    pub foreground: String,
    pub background: String,
}

impl InlineCodeColors {
    pub fn foreground(&self) -> Color {
        UiTheme::parse_color(&self.foreground)
    }
    pub fn background(&self) -> Color {
        UiTheme::parse_color(&self.background)
    }
}

/// 境界線の色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderColors {
    pub primary: String,
    pub secondary: String,
}

impl BorderColors {
    pub fn primary(&self) -> Color {
        UiTheme::parse_color(&self.primary)
    }
    pub fn secondary(&self) -> Color {
        UiTheme::parse_color(&self.secondary)
    }
}

/// テキストの色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextColors {
    pub primary: String,
    pub secondary: String,
    pub muted: String,
}

impl TextColors {
    pub fn primary(&self) -> Color {
        UiTheme::parse_color(&self.primary)
    }
    pub fn secondary(&self) -> Color {
        UiTheme::parse_color(&self.secondary)
    }
    pub fn muted(&self) -> Color {
        UiTheme::parse_color(&self.muted)
    }
}

/// ToCの色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TocColors {
    pub normal: String,
    pub selected: String,
    pub highlight_bg: String,
}

impl TocColors {
    pub fn normal(&self) -> Color {
        UiTheme::parse_color(&self.normal)
    }
    pub fn selected(&self) -> Color {
        UiTheme::parse_color(&self.selected)
    }
    pub fn highlight_bg(&self) -> Color {
        UiTheme::parse_color(&self.highlight_bg)
    }
}

/// ステータスバーの色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusBarColors {
    pub background: String,
    pub foreground: String,
    pub accent: String,
}

impl StatusBarColors {
    pub fn background(&self) -> Color {
        UiTheme::parse_color(&self.background)
    }
    pub fn foreground(&self) -> Color {
        UiTheme::parse_color(&self.foreground)
    }
    pub fn accent(&self) -> Color {
        UiTheme::parse_color(&self.accent)
    }
}

/// レイアウト設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutSettings {
    pub wrap_text: bool,
    pub toc_width_percent: u8,
    pub code_block_width_percent: u8,
}

impl LayoutSettings {
    pub fn wrap_text(&self) -> bool {
        self.wrap_text
    }
    pub fn toc_width_percent(&self) -> u8 {
        self.toc_width_percent
    }
    pub fn code_block_width_percent(&self) -> u8 {
        self.code_block_width_percent
    }
}

/// UIテーマ構造体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiTheme {
    /// 見出しの色
    pub heading: HeadingColors,
    /// コードブロックの色
    pub code: CodeColors,
    /// リストの色
    pub list: ListColors,
    /// 引用の色
    pub blockquote: BlockquoteColors,
    /// アラートの色
    pub alert: AlertColors,
    /// テーブルの色
    pub table: TableColors,
    /// インラインコードの色
    pub inline_code: InlineCodeColors,
    /// 境界線の色
    pub border: BorderColors,
    /// テキストの色
    pub text: TextColors,
    /// ToCの色
    pub toc: TocColors,
    /// ステータスバーの色
    pub status_bar: StatusBarColors,
    /// レイアウト設定
    pub layout: LayoutSettings,
}

impl UiTheme {
    /// 文字列をratatui::Colorに変換
    pub fn parse_color(s: &str) -> Color {
        match s.to_lowercase().as_str() {
            "black" => Color::Black,
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            "gray" | "grey" => Color::Gray,
            "darkgray" | "darkgrey" => Color::DarkGray,
            "lightred" => Color::LightRed,
            "lightgreen" => Color::LightGreen,
            "lightyellow" => Color::LightYellow,
            "lightblue" => Color::LightBlue,
            "lightmagenta" => Color::LightMagenta,
            "lightcyan" => Color::LightCyan,
            "white" => Color::White,
            _ => Color::White, // デフォルト
        }
    }

    /// デフォルトのダークテーマ
    pub fn dark() -> Self {
        Self {
            heading: HeadingColors {
                h1: "Cyan".to_string(),
                h2: "LightCyan".to_string(),
                h3: "Blue".to_string(),
                h4: "LightBlue".to_string(),
                h5: "Gray".to_string(),
                h6: "DarkGray".to_string(),
            },
            code: CodeColors {
                border: "DarkGray".to_string(),
                lang_label: "Magenta".to_string(),
            },
            list: ListColors {
                bullet: "Green".to_string(),
                checked: "Green".to_string(),
                unchecked: "Yellow".to_string(),
            },
            blockquote: BlockquoteColors {
                border: "Yellow".to_string(),
                text: "LightYellow".to_string(),
            },
            alert: AlertColors {
                note: AlertTypeColors {
                    border: "Blue".to_string(),
                    text: "LightBlue".to_string(),
                    icon: "Blue".to_string(),
                },
                tip: AlertTypeColors {
                    border: "Green".to_string(),
                    text: "LightGreen".to_string(),
                    icon: "Green".to_string(),
                },
                important: AlertTypeColors {
                    border: "Magenta".to_string(),
                    text: "LightMagenta".to_string(),
                    icon: "Magenta".to_string(),
                },
                warning: AlertTypeColors {
                    border: "Yellow".to_string(),
                    text: "LightYellow".to_string(),
                    icon: "Yellow".to_string(),
                },
                caution: AlertTypeColors {
                    border: "Red".to_string(),
                    text: "LightRed".to_string(),
                    icon: "Red".to_string(),
                },
            },
            table: TableColors {
                border: "Blue".to_string(),
                header: "Cyan".to_string(),
                cell: "White".to_string(),
            },
            inline_code: InlineCodeColors {
                foreground: "Yellow".to_string(),
                background: "DarkGray".to_string(),
            },
            border: BorderColors {
                primary: "Gray".to_string(),
                secondary: "DarkGray".to_string(),
            },
            text: TextColors {
                primary: "White".to_string(),
                secondary: "Gray".to_string(),
                muted: "DarkGray".to_string(),
            },
            toc: TocColors {
                normal: "White".to_string(),
                selected: "Black".to_string(),
                highlight_bg: "DarkGray".to_string(),
            },
            status_bar: StatusBarColors {
                background: "DarkGray".to_string(),
                foreground: "White".to_string(),
                accent: "Cyan".to_string(),
            },
            layout: LayoutSettings {
                wrap_text: false,
                toc_width_percent: 25,
                code_block_width_percent: 85,
            },
        }
    }

    /// デフォルトのライトテーマ
    pub fn light() -> Self {
        Self {
            heading: HeadingColors {
                h1: "Blue".to_string(),
                h2: "DarkGray".to_string(),
                h3: "Black".to_string(),
                h4: "Gray".to_string(),
                h5: "DarkGray".to_string(),
                h6: "Gray".to_string(),
            },
            code: CodeColors {
                border: "Gray".to_string(),
                lang_label: "Magenta".to_string(),
            },
            list: ListColors {
                bullet: "Green".to_string(),
                checked: "Green".to_string(),
                unchecked: "Yellow".to_string(),
            },
            blockquote: BlockquoteColors {
                border: "Gray".to_string(),
                text: "Black".to_string(),
            },
            alert: AlertColors {
                note: AlertTypeColors {
                    border: "Blue".to_string(),
                    text: "DarkGray".to_string(),
                    icon: "Blue".to_string(),
                },
                tip: AlertTypeColors {
                    border: "Green".to_string(),
                    text: "DarkGray".to_string(),
                    icon: "Green".to_string(),
                },
                important: AlertTypeColors {
                    border: "Magenta".to_string(),
                    text: "DarkGray".to_string(),
                    icon: "Magenta".to_string(),
                },
                warning: AlertTypeColors {
                    border: "Yellow".to_string(),
                    text: "DarkGray".to_string(),
                    icon: "Yellow".to_string(),
                },
                caution: AlertTypeColors {
                    border: "Red".to_string(),
                    text: "DarkGray".to_string(),
                    icon: "Red".to_string(),
                },
            },
            table: TableColors {
                border: "Gray".to_string(),
                header: "Black".to_string(),
                cell: "Black".to_string(),
            },
            inline_code: InlineCodeColors {
                foreground: "Black".to_string(),
                background: "LightGray".to_string(),
            },
            border: BorderColors {
                primary: "Gray".to_string(),
                secondary: "LightGray".to_string(),
            },
            text: TextColors {
                primary: "Black".to_string(),
                secondary: "Gray".to_string(),
                muted: "LightGray".to_string(),
            },
            toc: TocColors {
                normal: "Black".to_string(),
                selected: "White".to_string(),
                highlight_bg: "Gray".to_string(),
            },
            status_bar: StatusBarColors {
                background: "LightGray".to_string(),
                foreground: "Black".to_string(),
                accent: "Blue".to_string(),
            },
            layout: LayoutSettings {
                wrap_text: false,
                toc_width_percent: 25,
                code_block_width_percent: 85,
            },
        }
    }

    /// テーマをファイルから読み込む
    pub fn from_file(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let theme: UiTheme = toml::from_str(&content)?;
        Ok(theme)
    }

    /// テーマをファイルに保存する
    pub fn save_to_file(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

/// テーママネージャー
pub struct ThemeManager {
    themes: HashMap<String, UiTheme>,
    current_theme: String,
}

impl ThemeManager {
    /// 新しいテーママネージャーを作成
    pub fn new() -> Self {
        let mut themes = HashMap::new();
        themes.insert("dark".to_string(), UiTheme::dark());
        themes.insert("light".to_string(), UiTheme::light());

        Self {
            themes,
            current_theme: "dark".to_string(),
        }
    }

    /// テーマを追加
    pub fn add_theme(&mut self, name: String, theme: UiTheme) {
        self.themes.insert(name, theme);
    }

    /// テーマを削除
    pub fn remove_theme(&mut self, name: &str) {
        self.themes.remove(name);
    }

    /// 現在のテーマを取得
    pub fn current_theme(&self) -> UiTheme {
        self.themes
            .get(&self.current_theme)
            .cloned()
            .unwrap_or_else(UiTheme::dark)
    }

    /// 現在のテーマ名を取得
    pub fn current_theme_name(&self) -> &str {
        &self.current_theme
    }

    /// テーマを設定
    pub fn set_theme(&mut self, name: &str) -> bool {
        if self.themes.contains_key(name) {
            self.current_theme = name.to_string();
            true
        } else {
            false
        }
    }

    /// 利用可能なテーマ一覧を取得
    pub fn available_themes(&self) -> Vec<&String> {
        self.themes.keys().collect()
    }

    /// テーマをファイルから読み込んで追加
    pub fn load_theme_from_file(
        &mut self,
        name: String,
        path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let theme = UiTheme::from_file(path)?;
        self.add_theme(name, theme);
        Ok(())
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}
