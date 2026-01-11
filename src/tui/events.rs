use crossterm::event::{Event, EventStream, KeyEvent};
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

    pub async fn next_key(&mut self) -> anyhow::Result<Option<KeyEvent>> {
        if let Some(event) = self.stream.next().await {
            if let Event::Key(key) = event? {
                return Ok(Some(key));
            }
        }
        Ok(None)
    }
}
