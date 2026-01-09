use super::highlighter::{CodeHighlighter, StyledSpan};
use super::toc::TocEntry;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum ParsedLine {
    Heading {
        level: usize,
        text: String,
        line_num: usize,
    },
    Code {
        lang: Option<String>,
        content: String,
        highlighted: Vec<Vec<StyledSpan>>,
    },
    Text {
        content: String,
    },
    ListItem {
        indent: usize,
        content: String,
        checked: Option<bool>, // None = 通常のリスト, Some(true) = チェック済み, Some(false) = 未チェック
    },
    Table {
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
        alignments: Vec<Alignment>,
    },
    BlockQuote {
        content: String,
    },
    HorizontalRule,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Alignment {
    None,
    Left,
    Center,
    Right,
}

pub struct MarkdownDocument {
    pub path: PathBuf,
    pub content: String,
    pub parsed_lines: Vec<ParsedLine>,
    pub toc: Vec<TocEntry>,
}

impl MarkdownDocument {
    pub fn parse(
        path: PathBuf,
        content: String,
        highlighter: &CodeHighlighter,
    ) -> anyhow::Result<Self> {
        let mut parsed_lines = Vec::new();
        let mut toc = Vec::new();

        // テーブルや他の拡張機能を有効にする
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);

        let parser = Parser::new_ext(&content, options);
        let mut current_line = 0;

        let mut in_heading = false;
        let mut heading_level = 0;
        let mut heading_text = String::new();

        let mut in_code_block = false;
        let mut code_lang: Option<String> = None;
        let mut code_content = String::new();

        let mut in_list = false;
        let mut list_depth: usize = 0;
        let mut list_item_stack: Vec<(String, Option<bool>, usize)> = Vec::new(); // (content, checked, indent)のスタック

        let mut in_blockquote = false;
        let mut blockquote_content = String::new();

        let mut in_table = false;
        let mut in_table_head = false;
        let mut table_headers: Vec<String> = Vec::new();
        let mut table_rows: Vec<Vec<String>> = Vec::new();
        let mut current_row: Vec<String> = Vec::new();
        let mut current_cell = String::new();
        let mut table_alignments: Vec<Alignment> = Vec::new();

        let mut current_text = String::new();

        for event in parser {
            match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    if !current_text.is_empty() {
                        parsed_lines.push(ParsedLine::Text {
                            content: std::mem::take(&mut current_text),
                        });
                    }
                    in_heading = true;
                    heading_level = match level {
                        HeadingLevel::H1 => 1,
                        HeadingLevel::H2 => 2,
                        HeadingLevel::H3 => 3,
                        HeadingLevel::H4 => 4,
                        HeadingLevel::H5 => 5,
                        HeadingLevel::H6 => 6,
                    };
                    heading_text.clear();
                }
                Event::End(TagEnd::Heading(_)) => {
                    in_heading = false;
                    let line_num = current_line;
                    parsed_lines.push(ParsedLine::Heading {
                        level: heading_level,
                        text: heading_text.clone(),
                        line_num,
                    });
                    toc.push(TocEntry {
                        level: heading_level,
                        title: heading_text.clone(),
                        line_number: line_num,
                    });
                    current_line += 1;
                }
                Event::Start(Tag::CodeBlock(kind)) => {
                    if !current_text.is_empty() {
                        parsed_lines.push(ParsedLine::Text {
                            content: std::mem::take(&mut current_text),
                        });
                    }
                    in_code_block = true;
                    code_lang = match kind {
                        pulldown_cmark::CodeBlockKind::Fenced(lang) => {
                            if lang.is_empty() {
                                None
                            } else {
                                Some(lang.to_string())
                            }
                        }
                        pulldown_cmark::CodeBlockKind::Indented => None,
                    };
                    code_content.clear();
                }
                Event::End(TagEnd::CodeBlock) => {
                    in_code_block = false;
                    let highlighted = highlighter.highlight(&code_content, code_lang.as_deref());
                    parsed_lines.push(ParsedLine::Code {
                        lang: code_lang.clone(),
                        content: code_content.clone(),
                        highlighted,
                    });
                    current_line += code_content.lines().count() + 2;
                }
                Event::Start(Tag::List(_)) => {
                    if !current_text.is_empty() {
                        parsed_lines.push(ParsedLine::Text {
                            content: std::mem::take(&mut current_text),
                        });
                    }
                    
                    // ネストされたリストが開始する前に、親アイテムの内容を出力
                    if in_list && !list_item_stack.is_empty() {
                        if let Some(item) = list_item_stack.last_mut() {
                            if !item.0.trim().is_empty() {

                                parsed_lines.push(ParsedLine::ListItem {
                                    indent: item.2,
                                    content: item.0.trim().to_string(),
                                    checked: item.1,
                                });
                                // 出力済みなので内容をクリア（インデントレベルは保持）
                                item.0.clear();
                            }
                        }
                    }
                    
                    in_list = true;
                    list_depth += 1;
                }
                Event::End(TagEnd::List(_)) => {
                    list_depth = list_depth.saturating_sub(1);
                    if list_depth == 0 {
                        in_list = false;
                    }
                }
                Event::Start(Tag::Item) => {
                    // 新しいアイテムをスタックにプッシュ
                    let indent = list_depth.saturating_sub(1);
                    list_item_stack.push((String::new(), None, indent));
                }
                Event::End(TagEnd::Item) => {
                    if in_list {
                        if let Some((content, checked, indent)) = list_item_stack.pop() {
                            // 内容が空でない場合のみ出力（既に出力済みの場合は空）
                            if !content.trim().is_empty() {

                                parsed_lines.push(ParsedLine::ListItem {
                                    indent,
                                    content: content.trim().to_string(),
                                    checked,
                                });
                            }
                        }
                    }
                }
                Event::TaskListMarker(checked) => {
                    // 現在のアイテムのcheckedフラグを設定
                    if let Some(item) = list_item_stack.last_mut() {
                        item.1 = Some(checked);
                    }
                }
                Event::Start(Tag::BlockQuote(_)) => {
                    if !current_text.is_empty() {
                        parsed_lines.push(ParsedLine::Text {
                            content: std::mem::take(&mut current_text),
                        });
                    }
                    in_blockquote = true;
                    blockquote_content.clear();
                }
                Event::End(TagEnd::BlockQuote(_)) => {
                    in_blockquote = false;
                    parsed_lines.push(ParsedLine::BlockQuote {
                        content: std::mem::take(&mut blockquote_content),
                    });
                }
                Event::Text(text) => {
                    if in_heading {
                        heading_text.push_str(&text);
                    } else if in_code_block {
                        code_content.push_str(&text);
                    } else if in_table {
                        current_cell.push_str(&text);
                    } else if in_list {
                        // スタックの一番上のアイテムにテキストを追加
                        if let Some(item) = list_item_stack.last_mut() {
                            item.0.push_str(&text);
                        }
                    } else if in_blockquote {
                        blockquote_content.push_str(&text);
                    } else {
                        current_text.push_str(&text);
                    }
                }
                Event::Code(code) => {
                    let code_str = format!("⟨INLINE_CODE⟩{}⟨/INLINE_CODE⟩", code);
                    if in_heading {
                        heading_text.push_str(&code_str);
                    } else if in_table {
                        current_cell.push_str(&code_str);
                    } else if in_list {
                        // スタックの一番上のアイテムにコードを追加
                        if let Some(item) = list_item_stack.last_mut() {
                            item.0.push_str(&code_str);
                        }
                    } else if in_blockquote {
                        blockquote_content.push_str(&code_str);
                    } else {
                        current_text.push_str(&code_str);
                    }
                }
                Event::SoftBreak | Event::HardBreak => {
                    if in_heading {
                        heading_text.push(' ');
                    } else if in_code_block {
                        code_content.push('\n');
                    } else if in_table {
                        current_cell.push(' ');
                    } else if in_list {
                        // スタックの一番上のアイテムにスペースを追加
                        if let Some(item) = list_item_stack.last_mut() {
                            item.0.push(' ');
                        }
                    } else if in_blockquote {
                        blockquote_content.push('\n');
                    } else {
                        current_text.push(' ');
                    }
                }
                Event::Rule => {
                    if !current_text.is_empty() {
                        parsed_lines.push(ParsedLine::Text {
                            content: std::mem::take(&mut current_text),
                        });
                    }
                    parsed_lines.push(ParsedLine::HorizontalRule);
                }
                Event::Start(Tag::Table(alignments)) => {
                    if !current_text.is_empty() {
                        parsed_lines.push(ParsedLine::Text {
                            content: std::mem::take(&mut current_text),
                        });
                    }
                    in_table = true;
                    table_headers.clear();
                    table_rows.clear();
                    current_row.clear();
                    table_alignments = alignments
                        .iter()
                        .map(|a| match a {
                            pulldown_cmark::Alignment::None => Alignment::None,
                            pulldown_cmark::Alignment::Left => Alignment::Left,
                            pulldown_cmark::Alignment::Center => Alignment::Center,
                            pulldown_cmark::Alignment::Right => Alignment::Right,
                        })
                        .collect();
                }
                Event::End(TagEnd::Table) => {
                    in_table = false;
                    parsed_lines.push(ParsedLine::Table {
                        headers: std::mem::take(&mut table_headers),
                        rows: std::mem::take(&mut table_rows),
                        alignments: std::mem::take(&mut table_alignments),
                    });
                }
                Event::Start(Tag::TableHead) => {
                    in_table_head = true;
                    current_row.clear();
                }
                Event::End(TagEnd::TableHead) => {
                    in_table_head = false;
                    table_headers = std::mem::take(&mut current_row);
                }
                Event::Start(Tag::TableRow) => {
                    if !in_table_head {
                        current_row.clear();
                    }
                }
                Event::End(TagEnd::TableRow) => {
                    if !in_table_head {
                        table_rows.push(std::mem::take(&mut current_row));
                    }
                }
                Event::Start(Tag::TableCell) => {
                    current_cell.clear();
                }
                Event::End(TagEnd::TableCell) => {
                    current_row.push(std::mem::take(&mut current_cell));
                }
                _ => {}
            }
        }

        if !current_text.is_empty() {
            parsed_lines.push(ParsedLine::Text {
                content: current_text,
            });
        }

        Ok(MarkdownDocument {
            path,
            content,
            parsed_lines,
            toc,
        })
    }
}
