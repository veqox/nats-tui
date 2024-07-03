use ratatui::{
    layout::{Alignment, Rect},
    widgets::{
        block::{Block, Position, Title},
        Borders, Paragraph, Wrap,
    },
    Frame,
};
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

    pub fn render(&mut self, frame: &mut Frame, area: Rect, subject: &String, messages: &[Bytes]) {
        let Some(message) = messages.last() else {
            return;
        };

        let title = format!("Payload (Bytes: {})", message.len());
        let message = String::from_utf8_lossy(message);

        let widget = Paragraph::new(message).wrap(Wrap { trim: (false) }).block(
            Block::new().borders(Borders::ALL).title(
                Title::from(title)
                    .position(Position::Top)
                    .alignment(Alignment::Center),
            ),
        );

        frame.render_widget(widget, area)
    }
}
