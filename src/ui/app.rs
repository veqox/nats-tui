use std::io::Result;
use std::collections::HashSet;

use crate::{
    nats::client::Client,
    ui::tui::*,
};

use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use ratatui::{
    crossterm::event::KeyCode,
    layout::{Layout, Constraint, Direction},
    widgets::{Block, List, Paragraph, Borders},
};

pub struct App {
    tick_rate: f64,
    frame_rate: f64,
}

impl App {
    pub fn new(tick_rate: f64, frame_rate: f64) -> Self {
        Self {
            tick_rate,
            frame_rate
        }
    }

    pub async fn run(&mut self, mut client: Client) -> Result<()> {
        // TODO: clean up client situation
        let cancel_token = CancellationToken::new();
        let copy = cancel_token.clone();

        // TODO: connecting animation
        client.subscribe(String::from(">")).await.unwrap();

        let mut seen = HashSet::new();
        let (action_tx, mut action_rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = copy.cancelled() => {
                        break;
                    }
                    maybe_msg = client.next_msg() => {
                            if let Ok(msg) = maybe_msg {

                            if seen.contains(&msg) {
                                continue;
                            }
                            seen.insert(msg.clone());
                            action_tx.send(msg).unwrap();
                        }
                    }
                }
            }
        });

        let mut tui = Tui::init(self.tick_rate, self.frame_rate).unwrap();
        let mut subjects = Vec::new();

        loop {
            if let Some(ev) = tui.next().await {
                match ev {
                    TuiEvent::Tick => (),
                    TuiEvent::Render => { 
                        tui.terminal.draw(|frame| {
                            let layout = Layout::default()
                                .direction(Direction::Horizontal)
                                .constraints(vec![
                                    Constraint::Percentage(40),
                                    Constraint::Percentage(60),
                                ])
                                .split(frame.size());

                            frame.render_widget(
                                List::new(subjects.clone())
                                    .block(Block::new().borders(Borders::ALL)),
                                layout[0]);

                            frame.render_widget(
                                Paragraph::new("Right")
                                    .block(Block::new().borders(Borders::ALL)),
                                layout[1]);
                        })?;
                    }
                    TuiEvent::Key(key) => {
                        if key.code == KeyCode::Char('q') {
                            cancel_token.cancel();
                            tui.exit().unwrap();
                            break;
                        }
                    }
                    TuiEvent::Error => (),
                }
            }

            while let Ok(msg) = action_rx.try_recv() {
                subjects.push(msg);
            }
        }

        Ok(())
    }
}
