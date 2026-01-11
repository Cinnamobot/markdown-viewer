use crate::markdown::MarkdownDocument;
use crate::tui::ui::calculate_toc_width;
use crate::tui::ThemeManager;
use crossterm::event::{KeyCode, KeyModifiers};

pub struct App<'a> {
    pub document: MarkdownDocument,
    pub scroll_offset: usize,
    pub current_line: usize,
    pub show_toc: bool,
    pub toc_selected: usize,
    pub should_quit: bool,
    pub viewport_height: usize,
    pub theme_manager: &'a ThemeManager,
    pub search_mode: bool,
    pub search_query: String,
    pub search_results: Vec<usize>,
    pub current_search_index: usize,
    pub toc_width_cache: Option<u16>,
}

impl<'a> App<'a> {
    pub fn new(
        document: MarkdownDocument,
        show_toc: bool,
        initial_line: Option<usize>,
        theme_manager: &'a ThemeManager,
    ) -> Self {
        let scroll_offset = initial_line.unwrap_or(0);
        Self {
            document,
            scroll_offset,
            current_line: scroll_offset,
            show_toc,
            toc_selected: 0,
            should_quit: false,
            viewport_height: 0,
            theme_manager,
            search_mode: false,
            search_query: String::new(),
            search_results: Vec::new(),
            current_search_index: 0,
            toc_width_cache: None,
        }
    }

    pub fn update_document(&mut self, document: MarkdownDocument) {
        self.document = document;
        self.invalidate_toc_cache();

        // Adjust scroll_offset if it exceeds the new document length
        if self.scroll_offset >= self.document.parsed_lines.len() {
            self.scroll_offset = self.document.parsed_lines.len().saturating_sub(1);
        }

        // Adjust toc_selected if it exceeds the new TOC length
        if self.toc_selected >= self.document.toc.len() {
            self.toc_selected = self.document.toc.len().saturating_sub(1);
        }
    }

    pub fn handle_key(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        // Handle search mode input
        if self.search_mode && matches!(key, KeyCode::Char(_)) && modifiers.is_empty() {
            if let KeyCode::Char(c) = key {
                self.search_query.push(c);
            }
            return;
        }

        match (key, modifiers) {
            (KeyCode::Char('q'), _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                self.should_quit = true
            }
            (KeyCode::Down, _) | (KeyCode::Char('j'), _) => {
                if self.show_toc {
                    self.toc_down();
                } else {
                    self.scroll_down();
                }
            }
            (KeyCode::Up, _) | (KeyCode::Char('k'), _) => {
                if self.show_toc {
                    self.toc_up();
                } else {
                    self.scroll_up();
                }
            }
            (KeyCode::Char('t'), _) => self.toggle_toc(),
            (KeyCode::Enter, _) if self.show_toc => self.jump_to_heading(),
            (KeyCode::Enter, _) if self.search_mode => self.perform_search(),
            (KeyCode::Char('/'), _) if !self.search_mode => self.start_search(),
            (KeyCode::Char('n'), _) => self.next_search_result(),
            (KeyCode::Char('N'), KeyModifiers::SHIFT) => self.prev_search_result(),
            (KeyCode::Esc, _) if self.search_mode => self.end_search(),
            (KeyCode::Char('g'), _) => self.scroll_to_top(),
            (KeyCode::Char('G'), KeyModifiers::SHIFT) => self.scroll_to_bottom(),
            (KeyCode::PageDown, _) => self.page_down(),
            (KeyCode::PageUp, _) => self.page_up(),
            _ => {}
        }
    }

    fn scroll_down(&mut self) {
        if self.scroll_offset < self.document.parsed_lines.len().saturating_sub(1) {
            self.scroll_offset += 1;
        }
    }

    fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    fn page_down(&mut self) {
        self.scroll_offset = (self.scroll_offset + self.viewport_height)
            .min(self.document.parsed_lines.len().saturating_sub(1));
    }

    fn page_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(self.viewport_height);
    }

    fn scroll_to_top(&mut self) {
        self.scroll_offset = 0;
    }

    fn scroll_to_bottom(&mut self) {
        self.scroll_offset = self.document.parsed_lines.len().saturating_sub(1);
    }

    fn toggle_toc(&mut self) {
        self.show_toc = !self.show_toc;
    }

    fn toc_up(&mut self) {
        self.toc_selected = self.toc_selected.saturating_sub(1);
    }

    fn toc_down(&mut self) {
        if self.toc_selected < self.document.toc.len().saturating_sub(1) {
            self.toc_selected += 1;
        }
    }

    pub fn jump_to_heading(&mut self) {
        if let Some(entry) = self.document.toc.get(self.toc_selected) {
            self.scroll_offset = entry.line_number.saturating_sub(2);
            self.current_line = entry.line_number;
            self.show_toc = false;
        }
    }

    pub fn jump_to_heading_by_name(&mut self, heading: &str) {
        let heading_lower = heading.to_lowercase();
        if let Some((idx, _)) = self
            .document
            .toc
            .iter()
            .enumerate()
            .find(|(_, entry)| entry.title.to_lowercase().contains(&heading_lower))
        {
            self.toc_selected = idx;
            self.jump_to_heading();
        }
    }

    /// Get cached TOC width or calculate if not cached
    pub fn get_toc_width(&mut self, theme: &crate::tui::UiTheme, terminal_width: u16) -> u16 {
        if let Some(cached_width) = self.toc_width_cache {
            cached_width
        } else {
            let width = calculate_toc_width(self, theme, terminal_width);
            self.toc_width_cache = Some(width);
            width
        }
    }

    /// Clear TOC width cache when document changes
    pub fn invalidate_toc_cache(&mut self) {
        self.toc_width_cache = None;
    }

    pub fn start_search(&mut self) {
        self.search_mode = true;
        self.search_query.clear();
        self.search_results.clear();
        self.current_search_index = 0;
    }

    pub fn end_search(&mut self) {
        self.search_mode = false;
        self.search_query.clear();
        self.search_results.clear();
        self.current_search_index = 0;
    }

    pub fn perform_search(&mut self) {
        if self.search_query.is_empty() {
            self.end_search();
            return;
        }

        self.search_results.clear();
        let query_lower = self.search_query.to_lowercase();

        // Search in document content
        for (line_idx, parsed_line) in self.document.parsed_lines.iter().enumerate() {
            let content = match parsed_line {
                crate::markdown::ParsedLine::Text { content } => content.clone(),
                crate::markdown::ParsedLine::Code { content, .. } => content.clone(),
                crate::markdown::ParsedLine::ListItem { content, .. } => content.clone(),
                crate::markdown::ParsedLine::BlockQuote { content } => content.clone(),
                crate::markdown::ParsedLine::Alert { content, .. } => content.clone(),
                _ => continue,
            };

            if content.to_lowercase().contains(&query_lower) {
                self.search_results.push(line_idx);
            }
        }

        self.current_search_index = 0;
        if !self.search_results.is_empty() {
            self.scroll_offset = self.search_results[0];
        }
    }

    pub fn next_search_result(&mut self) {
        if self.search_results.is_empty() {
            return;
        }

        self.current_search_index = (self.current_search_index + 1) % self.search_results.len();
        self.scroll_offset = self.search_results[self.current_search_index];
    }

    pub fn prev_search_result(&mut self) {
        if self.search_results.is_empty() {
            return;
        }

        self.current_search_index = if self.current_search_index == 0 {
            self.search_results.len() - 1
        } else {
            self.current_search_index - 1
        };
        self.scroll_offset = self.search_results[self.current_search_index];
    }
}
