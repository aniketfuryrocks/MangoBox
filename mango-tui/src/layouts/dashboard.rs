use tui::widgets::{Block, Borders};
use tui::Frame;

use crate::app::AppBackend;

pub fn draw(frame: &mut Frame<AppBackend>) {
    let size = frame.size();
    let block = Block::default().title("Mango").borders(Borders::ALL);
    frame.render_widget(block, size);
}
