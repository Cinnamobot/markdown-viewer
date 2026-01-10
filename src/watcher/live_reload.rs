use notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_full::{new_debouncer, Debouncer, NoCache};
use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum ReloadEvent {
    FileChanged(PathBuf),
    Error(String),
}

pub struct LiveReloader {
    _debouncer: Debouncer<RecommendedWatcher, NoCache>,
    rx: mpsc::Receiver<ReloadEvent>,
}

impl LiveReloader {
    pub fn new(path: PathBuf) -> anyhow::Result<Self> {
        let (tx, rx) = mpsc::channel(100);

        let tx_clone = tx.clone();
        let mut debouncer = new_debouncer(
            Duration::from_millis(100),
            None,
            move |result: Result<
                Vec<notify_debouncer_full::DebouncedEvent>,
                Vec<notify::Error>,
            >| {
                let tx = tx_clone.clone();
                tokio::spawn(async move {
                    match result {
                        Ok(events) => {
                            for event in events {
                                if let Some(path) = event.paths.first() {
                                    let _ = tx.send(ReloadEvent::FileChanged(path.clone())).await;
                                }
                            }
                        }
                        Err(errors) => {
                            for error in errors {
                                let _ = tx.send(ReloadEvent::Error(error.to_string())).await;
                            }
                        }
                    }
                });
            },
        )?;

        debouncer.watch(&path, RecursiveMode::NonRecursive)?;

        Ok(Self {
            _debouncer: debouncer,
            rx,
        })
    }

    pub async fn next_event(&mut self) -> Option<ReloadEvent> {
        self.rx.recv().await
    }
}
