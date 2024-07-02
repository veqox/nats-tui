use std::collections::HashMap;
use std::io::Result;

use crate::{nats::client::Client, ui::tui::*};

use ratatui::style::{Color, Modifier, Style};
use ratatui::{
    crossterm::event::KeyCode,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{
        block::{Position, Title},
        Block, Borders, List, Paragraph,
    },
};
use tokio::sync::mpsc;
use tokio_util::bytes::Bytes;
use tokio_util::sync::CancellationToken;

pub struct App {
    tick_rate: f64,
    frame_rate: f64,
}

impl App {
    pub fn new(tick_rate: f64, frame_rate: f64) -> Self {
        Self {
            tick_rate,
            frame_rate,
        }
    }

    pub async fn run(&mut self, mut client: Client) -> Result<()> {
        // TODO: clean up client situation
        let cancel_token = CancellationToken::new();
        let copy = cancel_token.clone();

        // TODO: connecting animation
        client.subscribe(String::from(">")).await.unwrap();

        let (action_tx, mut action_rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = copy.cancelled() => {
                        break;
                    }
                    maybe_msg = client.next_msg() => {
                        if let Ok(msg) = maybe_msg {
                            action_tx.send(msg).unwrap();
                        }
                    }
                }
            }
        });

        let mut tui = Tui::init(self.tick_rate, self.frame_rate).unwrap();
        let mut messages: HashMap<String, Vec<Bytes>> = HashMap::new();

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

                            let subjects = messages
                                .iter()
                                .map(|(k, v)| format!("{}: {}", k, v.len()))
                                .collect::<Vec<String>>();

                            let subject_count = subjects.len();
                            let title = format!("Subjects ({})", subject_count);

                            frame.render_widget(
                                List::new(subjects)
                                    .highlight_style(
                                        Style::default()
                                            .fg(Color::Yellow)
                                            .bg(Color::Black)
                                            .add_modifier(Modifier::BOLD),
                                    )
                                    .block(
                                        Block::new().borders(Borders::ALL).title(
                                            Title::from(title)
                                                .position(Position::Top)
                                                .alignment(Alignment::Center),
                                        ),
                                    ),
                                layout[0],
                            );

                            frame.render_widget(
                                Paragraph::new("Right").block(Block::new().borders(Borders::ALL)),
                                layout[1],
                            );
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
                messages
                    .entry(msg.subject.to_string())
                    .or_default()
                    .push(msg.payload.clone());
            }
        }

        Ok(())
    }
}
