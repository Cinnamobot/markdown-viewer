use crate::markdown::MarkdownDocument;
use crossterm::event::{KeyCode, KeyModifiers};

pub struct App {
    pub document: MarkdownDocument,
    pub scroll_offset: usize,
    pub current_line: usize,
    pub show_toc: bool,
    pub toc_selected: usize,
    pub should_quit: bool,
    pub viewport_height: usize,
}

impl App {
    pub fn new(document: MarkdownDocument, show_toc: bool, initial_line: Option<usize>) -> Self {
        let scroll_offset = initial_line.unwrap_or(0);
        Self {
            document,
            scroll_offset,
            current_line: scroll_offset,
            show_toc,
            toc_selected: 0,
            should_quit: false,
            viewport_height: 0,
        }
    }

    pub fn update_document(&mut self, document: MarkdownDocument) {
        self.document = document;
        if self.scroll_offset >= self.document.parsed_lines.len() {
            self.scroll_offset = self.document.parsed_lines.len().saturating_sub(1);
        }
    }

    pub fn handle_key(&mut self, key: KeyCode, modifiers: KeyModifiers) {
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
}
