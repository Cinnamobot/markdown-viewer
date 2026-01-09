use crate::markdown::{Alignment, ParsedLine};
use crate::tui::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};
use syntect::highlighting::Color as SyntectColor;

pub fn render(frame: &mut Frame, app: &mut App) {
    let size = frame.area();
    app.viewport_height = size.height.saturating_sub(2) as usize;

    if app.show_toc {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(size);

        render_content(frame, chunks[0], app);
        render_toc(frame, chunks[1], app);
    } else {
        render_content(frame, size, app);
    }
}

fn render_content(frame: &mut Frame, area: Rect, app: &App) {
    let visible_count = area.height.saturating_sub(2) as usize;
    let visible_lines: Vec<Line> = app
        .document
        .parsed_lines
        .iter()
        .skip(app.scroll_offset)
        .take(visible_count)
        .flat_map(|line| parsed_line_to_ratatui_lines(line))
        .collect();

    let title = format!(" {} ", app.document.path.display());
    let paragraph = Paragraph::new(visible_lines)
        .block(Block::default().borders(Borders::ALL).title(title))
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

fn render_toc(frame: &mut Frame, area: Rect, app: &mut App) {
    let items: Vec<ListItem> = app
        .document
        .toc
        .iter()
        .map(|entry| {
            let indent = "  ".repeat(entry.level.saturating_sub(1));
            let content = format!("{}{}", indent, entry.title);
            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Table of Contents "),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    let mut list_state = ListState::default();
    list_state.select(Some(app.toc_selected));

    frame.render_stateful_widget(list, area, &mut list_state);
}

fn parsed_line_to_ratatui_lines(line: &ParsedLine) -> Vec<Line<'static>> {
    match line {
        ParsedLine::Heading { level, text, .. } => {
            let (style, prefix, suffix) = match level {
                1 => (
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                    "╔══ ",
                    " ══╗",
                ),
                2 => (
                    Style::default()
                        .fg(Color::LightCyan)
                        .add_modifier(Modifier::BOLD),
                    "▌ ",
                    "",
                ),
                3 => (
                    Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::BOLD),
                    "▸ ",
                    "",
                ),
                4 => (
                    Style::default()
                        .fg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD),
                    "  • ",
                    "",
                ),
                _ => (
                    Style::default()
                        .fg(Color::Gray)
                        .add_modifier(Modifier::BOLD),
                    "    ◦ ",
                    "",
                ),
            };

            vec![
                Line::from(""),
                Line::from(vec![Span::styled(format!("{prefix}{text}{suffix}"), style)]),
                Line::from(""),
            ]
        }
        ParsedLine::Code {
            lang,
            content,
            highlighted,
        } => {
            let lang_display = lang.as_deref().unwrap_or("text");
            let border_style = Style::default().fg(Color::DarkGray);
            let lang_style = Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD);
            
            // Fixed width for code blocks
            let block_width: usize = 80;
            
            // Header
            let lang_text = format!("[ {lang_display} ]");
            let lang_width = visible_text_len(&lang_text);
            let header_line_len = block_width.saturating_sub(2 + lang_width + 1); // ┌─ + lang + ┐
            
            let mut lines = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled("┌─", border_style),
                    Span::styled(lang_text, lang_style),
                    Span::styled("─".repeat(header_line_len), border_style),
                    Span::styled("┐", border_style),
                ]),
            ];

            // Content
            for highlighted_line in highlighted {
                let mut spans = vec![Span::styled("│ ", border_style)];
                let mut line_width = 2; // "│ "
                
                for styled_span in highlighted_line {
                    let text = &styled_span.text;
                    let span_width = visible_text_len(text);
                    let fg_color = syntect_to_ratatui_color(styled_span.style.foreground);
                    spans.push(Span::styled(text.clone(), Style::default().fg(fg_color)));
                    line_width += span_width;
                }
                
                // Add padding and right border
                if line_width < block_width - 1 {
                    let padding = block_width - 1 - line_width;
                    spans.push(Span::raw(" ".repeat(padding)));
                }
                spans.push(Span::styled("│", border_style));
                
                lines.push(Line::from(spans));
            }

            if highlighted.is_empty() {
                for line in content.lines() {
                    let line_width = visible_text_len(line);
                    let mut spans = vec![
                        Span::styled("│ ", border_style),
                        Span::raw(line.to_string()),
                    ];
                    
                    let current_width = 2 + line_width;
                    if current_width < block_width - 1 {
                        let padding = block_width - 1 - current_width;
                        spans.push(Span::raw(" ".repeat(padding)));
                    }
                    spans.push(Span::styled("│", border_style));
                    
                    lines.push(Line::from(spans));
                }
            }

            // Footer
            let footer_line_len = block_width.saturating_sub(3); // └─ + ┘ (total 3 chars width? └─ is 2, ┘ is 1)
            lines.push(Line::from(vec![
                Span::styled("└─", border_style),
                Span::styled("─".repeat(footer_line_len), border_style),
                Span::styled("┘", border_style),
            ]));
            lines.push(Line::from(""));
            lines
        }
        ParsedLine::Text { content } => {
            if content.trim().is_empty() {
                vec![Line::from("")]
            } else {
                let mut result: Vec<Line> = content
                    .split('\n')
                    .map(|line| {
                        // インラインコードを検出してハイライト
                        if line.contains("⟨INLINE_CODE⟩") {
                            parse_inline_code(line)
                        } else {
                            Line::from(Span::raw(line.to_string()))
                        }
                    })
                    .collect();
                // 段落の後に空行を追加
                result.push(Line::from(""));
                result
            }
        }
        ParsedLine::ListItem {
            indent,
            content,
            checked,
        } => {
            let indent_str = "  ".repeat(*indent);

            let (bullet, bullet_style) = match checked {
                Some(true) => (
                    format!("{indent_str}[✓] "),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Some(false) => (
                    format!("{indent_str}[ ] "),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                None => (
                    format!("{indent_str}● "),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
            };

            vec![Line::from(vec![
                Span::styled(bullet, bullet_style),
                Span::raw(content.clone()),
            ])]
        }
        ParsedLine::BlockQuote { content } => {
            let border_style = Style::default().fg(Color::Yellow);
            let text_style = Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::ITALIC);

            let lines: Vec<Line> = content
                .split('\n')
                .map(|line| {
                    Line::from(vec![
                        Span::styled("▐ ", border_style),
                        Span::styled(line.to_string(), text_style),
                    ])
                })
                .collect();
            let mut result = vec![Line::from("")];
            result.extend(lines);
            result.push(Line::from(""));
            result
        }
        ParsedLine::Table {
            headers,
            rows,
            alignments,
        } => render_table(headers, rows, alignments),
        ParsedLine::HorizontalRule => vec![
            Line::from(""),
            Line::from(Span::styled(
                "━".repeat(80),
                Style::default().fg(Color::Cyan),
            )),
            Line::from(""),
        ],
        ParsedLine::Empty => vec![Line::from("")],
    }
}

fn syntect_to_ratatui_color(color: SyntectColor) -> Color {
    Color::Rgb(color.r, color.g, color.b)
}

fn render_table(
    headers: &[String],
    rows: &[Vec<String>],
    alignments: &[Alignment],
) -> Vec<Line<'static>> {
    let border_style = Style::default().fg(Color::Blue);
    let header_style = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);
    let cell_style = Style::default().fg(Color::White);

    // 各列の最大幅を計算（マーカーを除外した可視文字数）
    let mut col_widths: Vec<usize> = headers.iter().map(|h| visible_text_len(h)).collect();
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            let visible_len = visible_text_len(cell);
            if i < col_widths.len() {
                col_widths[i] = col_widths[i].max(visible_len);
            } else {
                col_widths.push(visible_len);
            }
        }
    }
    // 最小幅を3、最大幅を30に制限
    col_widths.iter_mut().for_each(|w| {
        *w = (*w).clamp(3, 30);
    });

    let mut lines = Vec::new();
    lines.push(Line::from(""));

    // トップボーダー
    let top_border = format!(
        "┌{}┐",
        col_widths
            .iter()
            .map(|w| "─".repeat(w + 2))
            .collect::<Vec<_>>()
            .join("┬")
    );
    lines.push(Line::from(Span::styled(top_border, border_style)));

    // ヘッダー行
    let header_cells: Vec<Span> = headers
        .iter()
        .enumerate()
        .flat_map(|(i, header)| {
            let width = col_widths.get(i).copied().unwrap_or(10);
            let aligned = align_text(
                header,
                width,
                alignments.get(i).copied().unwrap_or(Alignment::Left),
            );
            
            let mut cell_spans = vec![Span::styled("│ ", border_style)];
            
            // インラインコードマーカーをチェック
            if aligned.contains("⟨INLINE_CODE⟩") {
                cell_spans.extend(parse_inline_code_to_spans(&aligned, header_style));
            } else {
                cell_spans.push(Span::styled(aligned, header_style));
            }
            
            cell_spans.push(Span::raw(" "));
            cell_spans
        })
        .chain(std::iter::once(Span::styled("│", border_style)))
        .collect();
    lines.push(Line::from(header_cells));

    // ヘッダー区切り
    let header_sep = format!(
        "├{}┤",
        col_widths
            .iter()
            .map(|w| "─".repeat(w + 2))
            .collect::<Vec<_>>()
            .join("┼")
    );
    lines.push(Line::from(Span::styled(header_sep, border_style)));

    // データ行
    for (row_idx, row) in rows.iter().enumerate() {
        let row_cells: Vec<Span> = (0..col_widths.len())
            .flat_map(|i| {
                let cell = row.get(i).map(|s| s.as_str()).unwrap_or("");
                let width = col_widths[i];
                let aligned = align_text(
                    cell,
                    width,
                    alignments.get(i).copied().unwrap_or(Alignment::Left),
                );
                
                let mut cell_spans = vec![Span::styled("│ ", border_style)];
                
                // インラインコードマーカーをチェック
                if aligned.contains("⟨INLINE_CODE⟩") {
                    cell_spans.extend(parse_inline_code_to_spans(&aligned, cell_style));
                } else {
                    cell_spans.push(Span::styled(aligned, cell_style));
                }
                
                cell_spans.push(Span::raw(" "));
                cell_spans
            })
            .chain(std::iter::once(Span::styled("│", border_style)))
            .collect();
        lines.push(Line::from(row_cells));

        // 行間の区切り（最後の行以外）
        if row_idx < rows.len() - 1 {
            let row_sep = format!(
                "├{}┤",
                col_widths
                    .iter()
                    .map(|w| "─".repeat(w + 2))
                    .collect::<Vec<_>>()
                    .join("┼")
            );
            lines.push(Line::from(Span::styled(row_sep, border_style)));
        }
    }

    // ボトムボーダー
    let bottom_border = format!(
        "└{}┘",
        col_widths
            .iter()
            .map(|w| "─".repeat(w + 2))
            .collect::<Vec<_>>()
            .join("┴")
    );
    lines.push(Line::from(Span::styled(bottom_border, border_style)));
    lines.push(Line::from(""));

    lines
}

// マーカーを除外した可視テキスト長を計算
pub fn visible_text_len(text: &str) -> usize {
    use unicode_width::UnicodeWidthStr;
    
    let open_marker = "⟨INLINE_CODE⟩";
    let close_marker = "⟨/INLINE_CODE⟩";
    
    let without_markers = text
        .replace(open_marker, "")
        .replace(close_marker, "");
        
    // Use width() instead of width_cjk() to treat ambiguous characters (like arrows) as width 1
    without_markers.width()
}

fn align_text(text: &str, width: usize, alignment: Alignment) -> String {
    let text_len = visible_text_len(text);
    if text_len >= width {
        // 長すぎる場合は切り詰める - マーカーを保持しながら可視文字数で切り詰め
        truncate_with_markers(text, width)
    } else {
        let padding = width - text_len;
        match alignment {
            Alignment::Left | Alignment::None => {
                format!("{text}{}", " ".repeat(padding))
            }
            Alignment::Right => {
                format!("{}{text}", " ".repeat(padding))
            }
            Alignment::Center => {
                let left_pad = padding / 2;
                let right_pad = padding - left_pad;
                format!("{}{text}{}", " ".repeat(left_pad), " ".repeat(right_pad))
            }
        }
    }
}

// マーカーを保持しながらテキストを切り詰める
pub fn truncate_with_markers(text: &str, max_visible: usize) -> String {
    use unicode_width::UnicodeWidthChar;
    
    let open_marker = "⟨INLINE_CODE⟩";
    let close_marker = "⟨/INLINE_CODE⟩";
    
    let mut result = String::new();
    let mut current_visible_width = 0;
    let mut i = 0;
    let mut in_code = false;
    let chars: Vec<char> = text.chars().collect();
    
    while i < chars.len() {
        let remaining: String = chars[i..].iter().collect();
        
        if remaining.starts_with(open_marker) {
            // Add marker without counting width
            result.push_str(open_marker);
            i += open_marker.chars().count();
            in_code = true;
        } else if remaining.starts_with(close_marker) {
            // Add marker without counting width
            result.push_str(close_marker);
            i += close_marker.chars().count();
            in_code = false;
        } else {
            // Check character width
            let ch = chars[i];
            // Use width() instead of width_cjk()
            let char_width = ch.width().unwrap_or(1);
            
            if current_visible_width + char_width > max_visible {
                break;
            }
            
            result.push(ch);
            current_visible_width += char_width;
            i += 1;
        }
    }
    
    // If we stopped inside a code block, add the closing marker
    if in_code {
        result.push_str(close_marker);
    }
    
    result
}

// テーブルセル用のインラインコードパーサー（ベーススタイルを保持）
fn parse_inline_code_to_spans(text: &str, base_style: Style) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    let mut current = String::new();
    
    let open_marker = "⟨INLINE_CODE⟩";
    let close_marker = "⟨/INLINE_CODE⟩";
    
    let mut i = 0;
    let chars: Vec<char> = text.chars().collect();
    
    while i < chars.len() {
        let remaining: String = chars[i..].iter().collect();
        if remaining.starts_with(open_marker) {
            // Push accumulated normal text with base style
            if !current.is_empty() {
                spans.push(Span::styled(current.clone(), base_style));
                current.clear();
            }
            
            i += open_marker.chars().count();
            
            let mut code_content = String::new();
            let mut found_close = false;
            while i < chars.len() {
                let remaining_inner: String = chars[i..].iter().collect();
                if remaining_inner.starts_with(close_marker) {
                    found_close = true;
                    i += close_marker.chars().count();
                    break;
                }
                code_content.push(chars[i]);
                i += 1;
            }
            
            // Add styled code span
            if found_close {
                spans.push(Span::styled(
                    code_content,
                    Style::default()
                        .fg(Color::Yellow)
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                ));
            } else {
                current.push_str(open_marker);
                current.push_str(&code_content);
            }
        } else {
            current.push(chars[i]);
            i += 1;
        }
    }
    
    if !current.is_empty() {
        spans.push(Span::styled(current, base_style));
    }
    
    if spans.is_empty() {
        spans.push(Span::styled("".to_string(), base_style));
    }
    
    spans
}

fn parse_inline_code(text: &str) -> Line<'static> {
    let mut spans = Vec::new();
    let mut current = String::new();
    
    let open_marker = "⟨INLINE_CODE⟩";
    let close_marker = "⟨/INLINE_CODE⟩";
    
    let mut i = 0;
    let chars: Vec<char> = text.chars().collect();
    
    while i < chars.len() {
        // Check for opening marker
        let remaining: String = chars[i..].iter().collect();
        if remaining.starts_with(open_marker) {
            // Push accumulated normal text
            if !current.is_empty() {
                spans.push(Span::raw(current.clone()));
                current.clear();
            }
            
            // Skip the opening marker
            i += open_marker.chars().count();
            
            // Collect code content until closing marker
            let mut code_content = String::new();
            let mut found_close = false;
            while i < chars.len() {
                let remaining_inner: String = chars[i..].iter().collect();
                if remaining_inner.starts_with(close_marker) {
                    found_close = true;
                    i += close_marker.chars().count();
                    break;
                }
                code_content.push(chars[i]);
                i += 1;
            }
            
            // Add styled code span
            if found_close {
                spans.push(Span::styled(
                    code_content,
                    Style::default()
                        .fg(Color::Yellow)
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                ));
            } else {
                // If no closing marker found, treat as normal text
                current.push_str(open_marker);
                current.push_str(&code_content);
            }
        } else {
            current.push(chars[i]);
            i += 1;
        }
    }
    
    // Add remaining text
    if !current.is_empty() {
        spans.push(Span::raw(current));
    }
    
    // If no spans were created, return empty line
    if spans.is_empty() {
        spans.push(Span::raw(""));
    }

    Line::from(spans)
}
