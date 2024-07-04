use std::time::Duration;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{
        block::{Block, Position, Title},
        Borders, List, Paragraph, Wrap,
    },
    Frame,
};
use serde_json::Value;
use tokio_util::bytes::Bytes;

#[derive(Debug)]
pub struct SubjectDetails {}

impl Default for SubjectDetails {
    fn default() -> Self {
        Self::new()
    }
}

impl SubjectDetails {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        _subject: &str,
        messages: &[Bytes],
        uptime: Duration,
    ) {
        let Some(message) = messages.last() else {
            return;
        };

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(area);

        let title = format!("Payload (Bytes: {})", message.len());
        let message = String::from_utf8_lossy(message);
        let parsed: Value = serde_json::from_str(&message).unwrap();
        let message = serde_json::to_string_pretty(&parsed).unwrap();
        let widget = Paragraph::new(message).wrap(Wrap { trim: (false) }).block(
            Block::new().borders(Borders::ALL).title(
                Title::from(title)
                    .position(Position::Top)
                    .alignment(Alignment::Center),
            ),
        );

        frame.render_widget(widget, layout[0]);

        let history_widget = List::new(
            messages
                .iter()
                .rev()
                .take(100)
                .map(|b| String::from_utf8_lossy(b)),
        )
        .block(
            Block::new().borders(Borders::ALL).title(
                Title::from(format!(
                    "History ({}, ~{:.2} messages per second)",
                    messages.len(),
                    messages.len() as f64 / uptime.as_secs() as f64,
                ))
                .position(Position::Top)
                .alignment(Alignment::Center),
            ),
        );

        frame.render_widget(history_widget, layout[1]);
    }
}
