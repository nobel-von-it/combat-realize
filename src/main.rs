
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
use crate::combat::Combat;
use crate::entity::{Monster, New, Player};

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    let mut t = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut combat = Combat::new(
        Player::new("held".to_string(), 100, 10, 10, 1),
        Monster::new("Ugly Bastard".to_string(), 50, 10, 10, 1)
    );

    let res = run(&mut t, &mut combat);

    disable_raw_mode()?;
    execute!(t.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    t.show_cursor()?;

    res?;
    Ok(())
}
fn run<B: Backend>(t: &mut Terminal<B>, combat: &mut Combat) -> anyhow::Result<()> {
    loop {
        t.draw(|f| combat.draw(f))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }
            match key.code {
                KeyCode::Esc => break,
                KeyCode::Enter => {
                    combat.hit()
                }
                _ => {}
            }
        }
    }
    Ok(())
}
