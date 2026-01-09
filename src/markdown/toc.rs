#[derive(Debug, Clone)]
pub struct TocEntry {
    pub level: usize,
    pub title: String,
    pub line_number: usize,
}
