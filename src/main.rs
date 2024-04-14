
mod combat;
mod entity;

use std::io::stdout;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    let mut t = Terminal::new(CrosstermBackend::new(stdout()))?;

    let res = run(&mut t);

    disable_raw_mode()?;
    execute!(t.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    t.show_cursor()?;

    res?;
    Ok(())
}
fn run<B: Backend>(t: &mut Terminal<B>) -> anyhow::Result<()> {
    loop {
        t.draw(|f| ui(f))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }
            match key.code {
                KeyCode::Esc => break,
                _ => {}
            }
        }
    }
    Ok(())
}

