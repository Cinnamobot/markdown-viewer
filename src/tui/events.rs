use crate::error::MdError;
use crossterm::event::{Event, EventStream, KeyEvent, KeyEventKind};
use futures::StreamExt;

pub struct EventHandler {
    stream: EventStream,
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            stream: EventStream::new(),
        }
    }

    pub async fn next_key(&mut self) -> Result<Option<KeyEvent>, MdError> {
        if let Some(event) = self.stream.next().await {
            if let Event::Key(key) = event? {
                // Ignore auto-repeat events: holding a key down (e.g. 't')
                // would otherwise rapidly toggle the TOC open/closed.
                // On Windows, key.kind is always set (Repeat for held keys).
                if key.kind == KeyEventKind::Repeat {
                    return Ok(None);
                }
                return Ok(Some(key));
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyModifiers};

    /// A repeat (held-key) event must be filtered out by the handler logic.
    #[test]
    fn test_repeat_key_is_ignored() {
        let repeat =
            KeyEvent::new_with_kind(KeyCode::Char('t'), KeyModifiers::NONE, KeyEventKind::Repeat);
        assert_eq!(repeat.kind, KeyEventKind::Repeat);
    }

    /// A normal press event keeps its kind.
    #[test]
    fn test_press_key_kind() {
        let press =
            KeyEvent::new_with_kind(KeyCode::Char('t'), KeyModifiers::NONE, KeyEventKind::Press);
        assert_eq!(press.kind, KeyEventKind::Press);
    }
}
