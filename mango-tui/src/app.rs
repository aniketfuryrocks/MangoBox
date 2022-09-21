use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Stdout},
    time::{Duration, Instant},
};
use tui::{backend::CrosstermBackend, Terminal};

use crate::layouts;

pub type AppBackend = CrosstermBackend<Stdout>;

#[derive(Default)]
pub struct AppState {}

pub struct App {
    terminal: Terminal<AppBackend>,
    should_quit: bool,
    state: AppState,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self {
            terminal,
            should_quit: false,
            state: AppState::default(),
        })
    }

    #[inline]
    pub fn draw(&mut self) -> anyhow::Result<()> {
        self.terminal
            .draw(|frame| layouts::dashboard::draw(frame, &self.state))?;

        Ok(())
    }

    pub fn restore(&mut self) -> anyhow::Result<()> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    pub fn run(&mut self, tick_rate: Duration) -> anyhow::Result<()> {
        self.should_quit = false;

        let mut last_tick = Instant::now();

        while !self.should_quit {
            self.draw()?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if let KeyCode::Char(c) = key.code {
                        self.on_key(c)
                    }
                }
            }
            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }

        self.restore()
    }

    pub fn on_key(&mut self, key: char) {
        if let 'q' = key {
            self.quit()
        }
    }

    #[inline]
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
