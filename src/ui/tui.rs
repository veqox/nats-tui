use std::io::stdout;

use futures::{FutureExt, StreamExt};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{Event as CrosstermEvent, KeyEvent, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    Terminal,
};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub enum TuiError {
    InitFailed(String),
    ExitFailed(String),
}

impl std::fmt::Display for TuiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for TuiError {
    fn description(&self) -> &str {
        match self {
            TuiError::InitFailed(msg) => msg,
            TuiError::ExitFailed(msg) => msg,
        }
    }
}

pub enum TuiEvent {
    Error,
    Tick,
    Render,
    Key(KeyEvent),
}

pub struct Tui {
    pub terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    pub event_tx: UnboundedSender<TuiEvent>,
    pub event_rx: UnboundedReceiver<TuiEvent>,
    pub tick_rate: f64,
    pub frame_rate: f64,
    pub cancellation_token: CancellationToken,
}

impl Tui {
    pub fn init(tick_rate: f64, frame_rate: f64) -> Result<Self, TuiError> {
        stdout()
            .execute(EnterAlternateScreen)
            .map_err(|_| TuiError::InitFailed("Failed to enter alternate screen".to_string()))?;
        enable_raw_mode()
            .map_err(|_| TuiError::InitFailed("Failed to enable alternate screen".to_string()))?;

        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))
            .map_err(|_| TuiError::InitFailed("Failed to init terminal".to_string()))?;
        terminal
            .clear()
            .map_err(|_| TuiError::InitFailed("Failed to clear terminal".to_string()))?;
        let (event_tx, event_rx) = mpsc::unbounded_channel();

        let mut tui = Self {
            terminal,
            event_tx,
            event_rx,
            tick_rate,
            frame_rate,
            cancellation_token: CancellationToken::new(),
        };

        tui.start();

        Ok(tui)
    }

    pub fn start(&mut self) {
        let tick_delay = std::time::Duration::from_secs_f64(1.0 / self.tick_rate);
        let render_delay = std::time::Duration::from_secs_f64(1.0 / self.frame_rate);

        let cancel_token = self.cancellation_token.clone();
        let event_tx = self.event_tx.clone();

        tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut tick_interval = tokio::time::interval(tick_delay);
            let mut render_interval = tokio::time::interval(render_delay);

            loop {
                let tick_delay = tick_interval.tick();
                let render_delay = render_interval.tick();
                let crossterm_event = reader.next().fuse();

                tokio::select! {
                    _ = cancel_token.cancelled() => {
                        break;
                    }
                    maybe_event = crossterm_event => {
                        match maybe_event {
                            Some(Ok(CrosstermEvent::Key(key))) => {
                                if key.kind == KeyEventKind::Press {
                                    event_tx.send(TuiEvent::Key(key)).unwrap();
                                }
                            },
                            Some(Err(_)) => event_tx.send(TuiEvent::Error).unwrap(),
                            None => (),
                            _ => ()
                        }
                    }
                    _ = tick_delay => event_tx.send(TuiEvent::Tick).unwrap(),
                    _ = render_delay => event_tx.send(TuiEvent::Render).unwrap(),
                }
            }
        });
    }

    pub fn exit(&mut self) -> Result<(), TuiError> {
        self.cancellation_token.cancel();
        self.terminal
            .flush()
            .map_err(|_| TuiError::ExitFailed("Failed to flush terminal".to_string()))?;
        stdout()
            .execute(LeaveAlternateScreen)
            .map_err(|_| TuiError::ExitFailed("Failed to leave alternate screen".to_string()))?;
        disable_raw_mode()
            .map_err(|_| TuiError::ExitFailed("Failed to disable raw mode".to_string()))
    }

    pub async fn next(&mut self) -> Option<TuiEvent> {
        self.event_rx.recv().await
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        self.exit().unwrap();
    }
}
