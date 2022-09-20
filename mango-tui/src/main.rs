use std::thread;
use std::time::Duration;

mod app;

fn main() -> anyhow::Result<()> {
    let mut terminal = app::App::new()?;

    // draw tui
    terminal.draw()?;

    // restore
    thread::sleep(Duration::from_millis(5000));
    terminal.restore()?;

    Ok(())
}
