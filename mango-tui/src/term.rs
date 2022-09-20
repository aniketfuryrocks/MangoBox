use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Stdout};
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal,
};

//pub type Result<T> = std::result::Result<T, io::Error>;

pub struct Term(Terminal<CrosstermBackend<Stdout>>);

impl Term {
    pub fn new() -> anyhow::Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        Ok(Self(Terminal::new(backend)?))
    }

    pub fn draw(&mut self) -> anyhow::Result<()> {
        self.0.draw(|f| {
            let size = f.size();
            let block = Block::default().title("Mango").borders(Borders::ALL);
            f.render_widget(block, size);
        })?;

        Ok(())
    }

    pub fn restore(&mut self) -> anyhow::Result<()> {
        disable_raw_mode()?;
        execute!(
            self.0.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.0.show_cursor()?;
        Ok(())
    }
}
