use tui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame,
};

use crate::app::{AppBackend, AppState};

pub fn draw(frame: &mut Frame<AppBackend>, _state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(80)].as_ref())
        .split(frame.size());
    let block = Block::default().title("Block").borders(Borders::ALL);
    frame.render_widget(block, chunks[0]);
    let block = Block::default().title("Block 2").borders(Borders::ALL);
    frame.render_widget(block, chunks[1]);
}
