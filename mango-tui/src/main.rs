use std::time::Duration;

mod app;
mod layouts;

fn main() -> anyhow::Result<()> {
    app::App::new()?.run(Duration::from_millis(250))
}
