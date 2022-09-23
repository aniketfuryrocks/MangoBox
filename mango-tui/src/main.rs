use std::thread;
use std::time::Duration;

mod term;

fn main() -> anyhow::Result<()> {
    let mut terminal = term::Term::new()?;

    // draw tui
    terminal.draw()?;

    // restore
    thread::sleep(Duration::from_millis(5000));
    terminal.restore()?;

    Ok(())
}
