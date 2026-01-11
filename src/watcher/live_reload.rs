use crate::error::MdError;
use notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_full::{new_debouncer, Debouncer, NoCache};
use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub enum ReloadEvent {
    FileChanged(PathBuf),
    Error(String),
}

pub struct LiveReloader {
    _debouncer: Debouncer<RecommendedWatcher, NoCache>,
    rx: mpsc::Receiver<ReloadEvent>,
    cancel_token: CancellationToken,
    _task_handles: Vec<tokio::task::JoinHandle<()>>,
}

impl LiveReloader {
    pub fn new(path: PathBuf) -> Result<Self, MdError> {
        let (tx, rx) = mpsc::channel(100);
        let cancel_token = CancellationToken::new();

        let tx_clone = tx.clone();
        let cancel_token_clone = cancel_token.clone();
        let mut debouncer = new_debouncer(
            Duration::from_millis(100),
            None,
            move |result: Result<
                Vec<notify_debouncer_full::DebouncedEvent>,
                Vec<notify::Error>,
            >| {
                let tx = tx_clone.clone();
                let cancel_token = cancel_token_clone.clone();
                tokio::spawn(async move {
                    // キャンセルされている場合は早期リターン
                    if cancel_token.is_cancelled() {
                        return;
                    }

                    match result {
                        Ok(events) => {
                            for event in events {
                                if let Some(path) = event.paths.first() {
                                    // チャネルがクローズされた場合はタスクを終了
                                    if tx
                                        .send(ReloadEvent::FileChanged(path.clone()))
                                        .await
                                        .is_err()
                                    {
                                        return;
                                    }
                                }
                            }
                        }
                        Err(errors) => {
                            for error in errors {
                                // チャネルがクローズされた場合はタスクを終了
                                if tx
                                    .send(ReloadEvent::Error(error.to_string()))
                                    .await
                                    .is_err()
                                {
                                    return;
                                }
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
            cancel_token,
            _task_handles: Vec::new(),
        })
    }

    pub async fn next_event(&mut self) -> Option<ReloadEvent> {
        self.rx.recv().await
    }
}

impl Drop for LiveReloader {
    fn drop(&mut self) {
        // キャンセルトークンをキャンセルして、実行中のタスクに終了を通知
        self.cancel_token.cancel();
    }
}
