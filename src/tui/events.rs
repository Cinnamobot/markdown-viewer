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
                // Only handle explicit key presses. On Windows, pressing a
                // key reports both Press and Release events, and holding it
                // reports Repeat events. Handling those would double-toggle
                // the TOC: press opens it, release instantly closes it.
                if key.kind != KeyEventKind::Press {
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

    #[test]
    fn test_press_kind_is_handled() {
        let press =
            KeyEvent::new_with_kind(KeyCode::Char('t'), KeyModifiers::NONE, KeyEventKind::Press);
        assert_eq!(press.kind, KeyEventKind::Press);
    }

    #[test]
    fn test_repeat_kind_is_ignored() {
        let repeat =
            KeyEvent::new_with_kind(KeyCode::Char('t'), KeyModifiers::NONE, KeyEventKind::Repeat);
        assert_eq!(repeat.kind, KeyEventKind::Repeat);
        assert_ne!(repeat.kind, KeyEventKind::Press);
    }

    #[test]
    fn test_release_kind_is_ignored() {
        let release =
            KeyEvent::new_with_kind(KeyCode::Char('t'), KeyModifiers::NONE, KeyEventKind::Release);
        assert_eq!(release.kind, KeyEventKind::Release);
        assert_ne!(release.kind, KeyEventKind::Press);
    }
}
