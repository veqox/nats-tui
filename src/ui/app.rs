use std::collections::HashMap;
use std::io::Result;

use crate::{nats::client::Client, ui::tui::*};

use ratatui::{
    crossterm::event::KeyCode,
    layout::{Constraint, Direction, Layout},
};
use tokio::sync::mpsc;
use tokio_util::bytes::Bytes;
use tokio_util::sync::CancellationToken;

use super::{subject_details::SubjectDetails, subject_overview::SubjectOverview};

pub struct App {
    tick_rate: f64,
    frame_rate: f64,

    subject_overview_widget: SubjectOverview,
    subject_details_widget: SubjectDetails,
}

impl App {
    pub fn new(tick_rate: f64, frame_rate: f64) -> Self {
        Self {
            tick_rate,
            frame_rate,

            subject_overview_widget: SubjectOverview::new(),
            subject_details_widget: SubjectDetails::new(),
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

                            self.subject_overview_widget
                                .render(frame, layout[0], &messages);

                            let Some(selected_subject) = self.subject_overview_widget.selected()
                            else {
                                return;
                            };

                            let Some((subject, messages)) = messages.iter().nth(selected_subject)
                            else {
                                return;
                            };

                            self.subject_details_widget
                                .render(frame, layout[1], subject, messages)
                        })?;
                    }
                    TuiEvent::Key(key) => {
                        if key.code == KeyCode::Char('q') {
                            cancel_token.cancel();
                            tui.exit().unwrap();
                            break;
                        }

                        self.subject_overview_widget.handle_key(key.code);
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
