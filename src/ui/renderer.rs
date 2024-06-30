use std::io::stdout;
use std::collections::HashSet;

use crate::nats::client::Client;

use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{Event as CrosstermEvent, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::Rect,
    style::Stylize,
    widgets::Paragraph,
    Terminal,
};

pub async fn render_loop(mut client: Client) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: connecting animation
    client.subscribe(">".to_string()).await?;

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let (event_tx, mut event_rx) = mpsc::unbounded_channel();
    let (sub_tx, mut sub_rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        let mut reader = crossterm::event::EventStream::new();

        loop {
            let crossterm_event = reader.next().fuse();

            tokio::select! {
                maybe_event = crossterm_event => {
                    match maybe_event {
                        Some(Ok(evt)) => {
                            match evt {
                                CrosstermEvent::Key(key) => {
                                    if key.kind == KeyEventKind::Press {
                                        event_tx.send(key).unwrap();
                                        if key.code == KeyCode::Char('q') {
                                            return;
                                        }
                                    }
                                }
                                _ => todo!(),
                            }
                        }
                        _ => {
                            return;
                        }
                    }
                }
            }
        }
    });

    let mut seen = HashSet::new();

    tokio::spawn(async move {
        while let Ok(msg) = client.next_msg().await {
            if seen.contains(&msg) {
                continue;
            }
            seen.insert(msg.clone());
            sub_tx.send(msg).unwrap();
        }
    });

    let mut subjects = Vec::new();

    'outer: loop {
        while let Ok(msg) = sub_rx.try_recv() {
            subjects.push(msg);

            terminal.draw(|frame| {
                for (i, sub) in subjects.iter().enumerate() {
                    let area = Rect::new(0, i as u16, frame.size().width, 1);
                    frame.render_widget(Paragraph::new(sub.to_string()).white(), area);
                }
            })?;
        }

        while let Ok(key) = event_rx.try_recv() {
            match key.code {
                KeyCode::Char('q') => break 'outer,
                _ => (),
            }
        } 
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

