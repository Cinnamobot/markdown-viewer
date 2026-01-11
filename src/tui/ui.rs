use crate::markdown::{Alignment, ParsedLine};
use crate::tui::app::App;
use crate::tui::UiTheme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};
use syntect::highlighting::Color as SyntectColor;
use unicode_width::UnicodeWidthChar;

/// Calculate optimal TOC width based on content and terminal size
pub fn calculate_toc_width<'a>(app: &App<'a>, theme: &UiTheme, terminal_width: u16) -> u16 {
    if app.document.toc.is_empty() {
        return 20; // Minimum width for empty TOC
    }

    // Calculate maximum heading length including indentation
    let max_heading_len = app
        .document
        .toc
        .iter()
        .map(|entry| {
            let indent = "  ".repeat(entry.level.saturating_sub(1));
            visible_text_len(&format!("{}{}>> ", indent, entry.title)) + 4 // padding and border
        })
        .max()
        .unwrap_or(20);

    // TOC title length
    let title_len = " Table of Contents ".len() + 4; // borders

    let content_len = max_heading_len.max(title_len);

    // Limit to reasonable bounds based on theme setting
    let max_percent = theme.layout.toc_width_percent() as usize;
    let max_toc_width = (terminal_width as usize * max_percent / 100).max(20);
    let min_toc_width = 20;

    (content_len as u16).clamp(min_toc_width, max_toc_width as u16)
}

/// Improved word wrapping function with Unicode support
fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    if text.is_empty() {
        return vec![String::new()];
    }

    let mut result = Vec::new();
    let mut current_line = String::new();

    // Split by whitespace, but preserve spaces within words
    for word in text.split_whitespace() {
        let word_len = visible_text_len(word);
        let current_len = visible_text_len(&current_line);

        if current_len + word_len < max_width || current_line.is_empty() {
            // Word fits on current line
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        } else {
            // Word doesn't fit
            if !current_line.is_empty() {
                result.push(current_line);
                current_line = String::new();
            }

            // Handle very long words by breaking them
            if word_len > max_width {
                let mut remaining = word;
                while !remaining.is_empty() {
                    let mut chunk_len = 0;
                    let mut chunk = String::new();

                    for ch in remaining.chars() {
                        let ch_width = ch.width().unwrap_or(1);
                        if chunk_len + ch_width > max_width {
                            break;
                        }
                        chunk.push(ch);
                        chunk_len += ch_width;
                    }

                    if chunk.is_empty() {
                        // Single character is too wide, add anyway
                        chunk.push(remaining.chars().next().unwrap());
                        remaining = &remaining[chunk.len()..];
                    } else {
                        remaining = &remaining[chunk.len()..];
                    }

                    result.push(chunk);
                }
            } else {
                current_line = word.to_string();
            }
        }
    }

    if !current_line.is_empty() {
        result.push(current_line);
    }

    if result.is_empty() {
        result.push(String::new());
    }

    result
}

/// Render status bar
fn render_status_bar<'a>(
    frame: &mut Frame,
    area: Rect,
    app: &App<'a>,
    theme_manager: &'a crate::tui::ThemeManager,
) {
    let theme = &theme_manager.current_theme();
    let status_text = if app.search_mode {
        format!(
            " Search: {} | {}/{} matches ",
            app.search_query,
            if app.search_results.is_empty() {
                0
            } else {
                app.current_search_index + 1
            },
            app.search_results.len()
        )
    } else {
        format!(
            " {} | Line {}/{} | Mode: {} | Theme: {} ",
            app.document.path.display(),
            app.current_line + 1,
            app.document.parsed_lines.len(),
            if app.show_toc { "TOC" } else { "View" },
            theme_manager.current_theme_name()
        )
    };

    let status_bar = Paragraph::new(status_text)
        .style(
            Style::default()
                .fg(theme.status_bar.foreground())
                .bg(theme.status_bar.background()),
        )
        .alignment(ratatui::layout::Alignment::Center);

    let status_area = Rect {
        x: area.x,
        y: area.y + area.height - 1,
        width: area.width,
        height: 1,
    };

    frame.render_widget(status_bar, status_area);
}

/// Render the TUI interface
pub fn render<'a>(
    frame: &mut Frame,
    app: &mut App<'a>,
    theme_manager: &'a crate::tui::ThemeManager,
) {
    let size = frame.area();
    app.viewport_height = size.height.saturating_sub(3) as usize; // -1 for status bar, -2 for borders
    let theme = &theme_manager.current_theme();

    if app.show_toc {
        // Calculate TOC width based on content (with caching)
        let toc_width = app.get_toc_width(theme, size.width);
        let content_width = size.width.saturating_sub(toc_width);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(content_width),
                Constraint::Length(toc_width),
            ])
            .split(size);

        render_content(frame, chunks[0], app, theme);
        render_toc(frame, chunks[1], app, theme);
    } else {
        render_content(frame, size, app, theme);
    }

    // Render status bar at the bottom
    render_status_bar(frame, size, app, theme_manager);
}

fn render_content<'a>(frame: &mut Frame, area: Rect, app: &App<'a>, theme: &UiTheme) {
    let visible_count = area.height.saturating_sub(2) as usize;

    // Handle empty document
    let visible_lines: Vec<Line> = if app.document.parsed_lines.is_empty() {
        vec![
            Line::from(""),
            Line::from(Span::styled(
                "  (Empty document)",
                Style::default().fg(theme.text.muted()),
            )),
        ]
    } else {
        app.document
            .parsed_lines
            .iter()
            .skip(app.scroll_offset)
            .take(visible_count)
            .flat_map(|line| parsed_line_to_ratatui_lines(line, theme, area.width as usize))
            .collect()
    };

    let title = format!(" {} ", app.document.path.display());
    let paragraph = Paragraph::new(visible_lines)
        .block(Block::default().borders(Borders::ALL).title(title))
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

fn render_toc<'a>(frame: &mut Frame, area: Rect, app: &mut App<'a>, theme: &UiTheme) {
    // Handle empty TOC
    let items: Vec<ListItem> = if app.document.toc.is_empty() {
        vec![ListItem::new(Span::styled(
            "  (No headings)",
            Style::default().fg(theme.text.muted()),
        ))]
    } else {
        app.document
            .toc
            .iter()
            .map(|entry| {
                let indent = "  ".repeat(entry.level.saturating_sub(1));
                let content = format!("{}{}", indent, entry.title);
                ListItem::new(content)
            })
            .collect()
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Table of Contents "),
        )
        .highlight_style(
            Style::default()
                .fg(theme.toc.selected())
                .bg(theme.toc.highlight_bg())
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    let mut list_state = ListState::default();
    // Only select if TOC is not empty
    if !app.document.toc.is_empty() {
        list_state.select(Some(app.toc_selected));
    }

    frame.render_stateful_widget(list, area, &mut list_state);
}

fn parsed_line_to_ratatui_lines(
    line: &ParsedLine,
    theme: &crate::tui::UiTheme,
    area_width: usize,
) -> Vec<Line<'static>> {
    match line {
        ParsedLine::Heading { level, text, .. } => {
            let (style, prefix, suffix) = match level {
                1 => (
                    Style::default()
                        .fg(theme.heading.h1())
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                    "╔══ ",
                    " ══╗",
                ),
                2 => (
                    Style::default()
                        .fg(theme.heading.h2())
                        .add_modifier(Modifier::BOLD),
                    "▌ ",
                    "",
                ),
                3 => (
                    Style::default()
                        .fg(theme.heading.h3())
                        .add_modifier(Modifier::BOLD),
                    "▸ ",
                    "",
                ),
                4 => (
                    Style::default()
                        .fg(theme.heading.h4())
                        .add_modifier(Modifier::BOLD),
                    "  • ",
                    "",
                ),
                _ => (
                    Style::default()
                        .fg(theme.heading.h6())
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
            let border_style = Style::default().fg(theme.code.border());
            let lang_style = Style::default()
                .fg(theme.code.lang_label())
                .add_modifier(Modifier::BOLD);

            // Responsive width for code blocks based on theme setting
            let available_width = area_width.saturating_sub(4); // Account for borders
            let percent = theme.layout.code_block_width_percent() as usize;
            let block_width: usize = (available_width * percent / 100).clamp(40, 120);

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
                let mut result: Vec<Line> = Vec::new();

                for line in content.split('\n') {
                    if theme.layout.wrap_text() && !line.is_empty() {
                        // Word wrapping for long lines
                        let wrapped_lines = wrap_text(line, area_width.saturating_sub(4));
                        for wrapped_line in wrapped_lines {
                            if wrapped_line.contains("⟨INLINE_CODE⟩") {
                                result.push(parse_inline_code(&wrapped_line, theme));
                            } else {
                                result.push(Line::from(Span::raw(wrapped_line)));
                            }
                        }
                    } else {
                        // No wrapping
                        if line.contains("⟨INLINE_CODE⟩") {
                            result.push(parse_inline_code(line, theme));
                        } else {
                            result.push(Line::from(Span::raw(line.to_string())));
                        }
                    }
                }

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
                        .fg(theme.list.checked())
                        .add_modifier(Modifier::BOLD),
                ),
                Some(false) => (
                    format!("{indent_str}[ ] "),
                    Style::default()
                        .fg(theme.list.unchecked())
                        .add_modifier(Modifier::BOLD),
                ),
                None => (
                    format!("{indent_str}● "),
                    Style::default()
                        .fg(theme.list.bullet())
                        .add_modifier(Modifier::BOLD),
                ),
            };

            let mut spans = vec![Span::styled(bullet, bullet_style)];

            // Check for inline code markers
            if content.contains("⟨INLINE_CODE⟩") {
                spans.extend(parse_inline_code_to_spans(content, Style::default(), theme));
            } else {
                spans.push(Span::raw(content.clone()));
            }

            vec![Line::from(spans)]
        }
        ParsedLine::BlockQuote { content } => {
            let border_style = Style::default().fg(theme.blockquote.border());
            let text_style = Style::default()
                .fg(theme.blockquote.text())
                .add_modifier(Modifier::ITALIC);

            let lines: Vec<Line> = content
                .split('\n')
                .map(|line| {
                    let mut spans = vec![Span::styled("▐ ", border_style)];

                    // Check for inline code markers
                    if line.contains("⟨INLINE_CODE⟩") {
                        spans.extend(parse_inline_code_to_spans(line, text_style, theme));
                    } else {
                        spans.push(Span::styled(line.to_string(), text_style));
                    }

                    Line::from(spans)
                })
                .collect();
            let mut result = vec![Line::from("")];
            result.extend(lines);
            result.push(Line::from(""));
            result
        }
        ParsedLine::Alert {
            alert_type,
            content,
        } => {
            use crate::markdown::parser::AlertType;

            let (icon, label, border_color, text_color) = match alert_type {
                AlertType::Note => (
                    "ℹ",
                    "NOTE",
                    theme.alert.note.border(),
                    theme.alert.note.text(),
                ),
                AlertType::Tip => (
                    "💡",
                    "TIP",
                    theme.alert.tip.border(),
                    theme.alert.tip.text(),
                ),
                AlertType::Important => (
                    "❗",
                    "IMPORTANT",
                    theme.alert.important.border(),
                    theme.alert.important.text(),
                ),
                AlertType::Warning => (
                    "⚠",
                    "WARNING",
                    theme.alert.warning.border(),
                    theme.alert.warning.text(),
                ),
                AlertType::Caution => (
                    "🛑",
                    "CAUTION",
                    theme.alert.caution.border(),
                    theme.alert.caution.text(),
                ),
            };

            let border_style = Style::default()
                .fg(border_color)
                .add_modifier(Modifier::BOLD);
            let text_style = Style::default().fg(text_color);

            // Calculate dynamic width based on content
            let header_prefix_len = visible_text_len(&format!("┏━━ {icon} {label} "));
            let footer_prefix_len = visible_text_len("┗");
            let side_border_len = visible_text_len("┃ ");

            let mut max_content_len = 0;
            for line in content.split('\n') {
                let line_len = visible_text_len(line);
                max_content_len = max_content_len.max(line_len);
            }

            let total_width = (header_prefix_len + max_content_len + side_border_len)
                .max(footer_prefix_len + 60)
                .min(area_width.saturating_sub(4));
            let border_fill_len = total_width.saturating_sub(header_prefix_len);

            let mut result = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled("┏━━ ", border_style),
                    Span::styled(format!("{icon} {label} "), border_style),
                    Span::styled("━".repeat(border_fill_len), border_style),
                ]),
            ];

            for line in content.split('\n') {
                let mut spans = vec![Span::styled("┃ ", border_style)];

                if line.contains("⟨INLINE_CODE⟩") {
                    spans.extend(parse_inline_code_to_spans(line, text_style, theme));
                } else {
                    spans.push(Span::styled(line.to_string(), text_style));
                }

                // Pad to align with border
                let current_len = visible_text_len(
                    &line
                        .replace("⟨INLINE_CODE⟩", "")
                        .replace("⟨/INLINE_CODE⟩", ""),
                );
                let padding_needed = max_content_len.saturating_sub(current_len);
                if padding_needed > 0 {
                    spans.push(Span::raw(" ".repeat(padding_needed)));
                }

                spans.push(Span::styled(" ", border_style)); // Right padding
                result.push(Line::from(spans));
            }

            let footer_line = format!(
                "┗{}",
                "━".repeat(total_width.saturating_sub(footer_prefix_len))
            );
            result.push(Line::from(Span::styled(footer_line, border_style)));
            result.push(Line::from(""));
            result
        }
        ParsedLine::Table {
            headers,
            rows,
            alignments,
        } => render_table(headers, rows, alignments, theme, area_width),
        ParsedLine::HorizontalRule => {
            let rule_width = area_width.saturating_sub(4).min(120); // Responsive, max 120
            vec![
                Line::from(""),
                Line::from(Span::styled(
                    "━".repeat(rule_width),
                    Style::default().fg(theme.border.primary()),
                )),
                Line::from(""),
            ]
        }
        ParsedLine::Image { alt_text, url } => vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "[Image: ".to_string(),
                    Style::default().fg(theme.border.primary()),
                ),
                Span::styled(
                    url.to_string(),
                    Style::default()
                        .fg(theme.text.secondary())
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled("]".to_string(), Style::default().fg(theme.border.primary())),
            ]),
            Line::from(vec![
                Span::styled("Alt: ".to_string(), Style::default().fg(theme.text.muted())),
                Span::styled(
                    alt_text.to_string(),
                    Style::default().fg(theme.text.secondary()),
                ),
            ]),
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
    theme: &UiTheme,
    area_width: usize,
) -> Vec<Line<'static>> {
    // Handle empty table (malformed Markdown)
    if headers.is_empty() {
        return vec![
            Line::from(""),
            Line::from(Span::styled(
                "  (Empty table)",
                Style::default().fg(theme.text.muted()),
            )),
            Line::from(""),
        ];
    }

    let border_style = Style::default().fg(theme.table.border());
    let header_style = Style::default()
        .fg(theme.table.header())
        .add_modifier(Modifier::BOLD);
    let cell_style = Style::default().fg(theme.table.cell());

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

    // Responsive column widths: distribute available space
    let total_content_width: usize = col_widths.iter().sum::<usize>() + col_widths.len() * 3; // +3 for " │ " padding
    let available_width = area_width.saturating_sub(4); // Account for outer borders
    let min_col_width = 3;
    let max_col_width = 50; // Increased from 30

    if total_content_width <= available_width {
        // Content fits, use natural widths but respect minimums
        col_widths.iter_mut().for_each(|w| {
            *w = (*w).max(min_col_width).min(max_col_width);
        });
    } else {
        // Content too wide, scale down proportionally
        let total_desired: usize = col_widths.iter().sum();
        let scale_factor = available_width as f64 / (total_desired + col_widths.len() * 3) as f64;

        for w in &mut col_widths {
            *w = ((*w as f64 * scale_factor) as usize)
                .max(min_col_width)
                .min(max_col_width);
        }
    }

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
                cell_spans.extend(parse_inline_code_to_spans(&aligned, header_style, theme));
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
                    cell_spans.extend(parse_inline_code_to_spans(&aligned, cell_style, theme));
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

    let without_markers = text.replace(open_marker, "").replace(close_marker, "");

    // Use width() instead of width_cjk() to treat ambiguous characters (like arrows) as width 1
    without_markers.width()
}

fn align_text(text: &str, width: usize, alignment: Alignment) -> String {
    let text_len = visible_text_len(text);
    let content = if text_len > width {
        // 長すぎる場合は切り詰める - マーカーを保持しながら可視文字数で切り詰め
        truncate_with_markers(text, width)
    } else {
        text.to_string()
    };

    let content_len = visible_text_len(&content);
    let padding = width.saturating_sub(content_len);

    match alignment {
        Alignment::Left | Alignment::None => {
            format!("{content}{}", " ".repeat(padding))
        }
        Alignment::Right => {
            format!("{}{content}", " ".repeat(padding))
        }
        Alignment::Center => {
            let left_pad = padding / 2;
            let right_pad = padding - left_pad;
            format!("{}{content}{}", " ".repeat(left_pad), " ".repeat(right_pad))
        }
    }
}

// マーカーを保持しながらテキストを切り詰める（O(n)アルゴリズム）
pub fn truncate_with_markers(text: &str, max_visible: usize) -> String {
    let open_marker = "⟨INLINE_CODE⟩";
    let close_marker = "⟨/INLINE_CODE⟩";

    let mut result = String::with_capacity(text.len().min(max_visible * 2));
    let mut current_visible_width = 0;
    let mut i = 0;
    let mut in_code = false;

    while i < text.len() {
        let remaining = &text[i..];

        if remaining.starts_with(open_marker) {
            result.push_str(open_marker);
            i += open_marker.len();
            in_code = true;
        } else if remaining.starts_with(close_marker) {
            result.push_str(close_marker);
            i += close_marker.len();
            in_code = false;
        } else {
            if let Some(ch) = text[i..].chars().next() {
                let char_width = match ch.width() {
                    Some(w) => w,
                    None => {
                        if ch.is_control() {
                            0
                        } else {
                            1
                        }
                    }
                };

                if current_visible_width + char_width > max_visible {
                    break;
                }

                result.push(ch);
                current_visible_width += char_width;
                i += ch.len_utf8();
            } else {
                break;
            }
        }
    }

    if in_code {
        result.push_str(close_marker);
    }

    result
}

// テーブルセル用のインラインコードパーサー（ベーススタイルを保持）
fn parse_inline_code_to_spans(
    text: &str,
    base_style: Style,
    theme: &crate::tui::UiTheme,
) -> Vec<Span<'static>> {
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
                        .fg(theme.inline_code.foreground())
                        .bg(theme.inline_code.background())
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

fn parse_inline_code(text: &str, theme: &UiTheme) -> Line<'static> {
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
                        .fg(theme.inline_code.foreground())
                        .bg(theme.inline_code.background())
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

#[cfg(test)]
mod test_inline_code_spans {
    use super::*;
    use ratatui::style::{Color, Modifier, Style};

    #[test]
    fn test_parse_inline_code_to_spans_basic() {
        let text = "⟨INLINE_CODE⟩test⟨/INLINE_CODE⟩";
        let base_style = Style::default().fg(Color::White);
        let theme = crate::tui::UiTheme::dark();
        let spans = parse_inline_code_to_spans(text, base_style, &theme);

        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].content, "test");
        let expected_style = Style::default()
            .fg(theme.inline_code.foreground())
            .bg(theme.inline_code.background())
            .add_modifier(Modifier::BOLD);
        assert_eq!(spans[0].style, expected_style);
    }

    #[test]
    fn test_parse_inline_code_to_spans_with_normal_text() {
        let text = "normal ⟨INLINE_CODE⟩code⟨/INLINE_CODE⟩ more";
        let base_style = Style::default().fg(Color::White);
        let theme = crate::tui::UiTheme::dark();
        let spans = parse_inline_code_to_spans(text, base_style, &theme);

        assert_eq!(spans.len(), 3);
        assert_eq!(spans[0].content, "normal ");
        assert_eq!(spans[0].style, base_style);

        assert_eq!(spans[1].content, "code");
        let expected_code_style = Style::default()
            .fg(theme.inline_code.foreground())
            .bg(theme.inline_code.background())
            .add_modifier(Modifier::BOLD);
        assert_eq!(spans[1].style, expected_code_style);

        assert_eq!(spans[2].content, " more");
        assert_eq!(spans[2].style, base_style);
    }
}
