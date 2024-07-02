use std::collections::HashMap;

use crossterm::event::KeyCode;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        block::{Position, Title},
        Block, Borders, List, ListItem, ListState,
    },
    Frame,
};
use tokio_util::bytes::Bytes;

#[derive(Debug)]
pub struct SubjectOverview {
    pub state: ListState,
}

impl Default for SubjectOverview {
    fn default() -> Self {
        Self::new()
    }
}

impl SubjectOverview {
    pub fn new() -> Self {
        Self {
            state: ListState::default(),
        }
    }

    pub fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        messages: &HashMap<String, Vec<Bytes>>,
    ) {
        let title = format!("Subjects ({})", messages.len());
        let messages = messages
            .iter()
            .map(|(subject, messages)| {
                ListItem::new(Line::from(vec![
                    Span::styled(subject, Style::default()),
                    Span::raw(" "),
                    Span::styled(
                        format!("({} messages)", messages.len()),
                        Style::new().fg(Color::DarkGray),
                    ),
                ]))
            })
            .collect::<Vec<ListItem>>();

        let widget = List::new(messages)
            .highlight_style(
                Style::new()
                    .bg(Color::LightBlue)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .block(
                Block::new().borders(Borders::ALL).title(
                    Title::from(title)
                        .position(Position::Top)
                        .alignment(Alignment::Center),
                ),
            );

        frame.render_stateful_widget(widget, area, &mut self.state)
    }

    pub fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Down => self.select_next(),
            KeyCode::Up => self.select_previous(),
            KeyCode::Char('j') => self.select_next(),
            KeyCode::Char('k') => self.select_previous(),
            _ => {}
        }
    }

    pub fn select_next(&mut self) {
        let next = self.state.selected().map_or(0, |i| i + 1);
        self.state.select(Some(next));
    }

    pub fn select_previous(&mut self) {
        self.state
            .select(Some(self.state.selected().unwrap_or(0).saturating_sub(1)));
    }

    pub fn reset_selection(&mut self) {
        self.state.select(None);
    }

    pub fn selected(&self) -> Option<usize> {
        self.state.selected()
    }
}
