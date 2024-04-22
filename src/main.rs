mod combat;
mod entity;

use std::io::stdout;

use crate::combat::Combat;
use crate::entity::{Action, Monster, New, Player};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    let mut t = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut combat = Combat::new(
        Player::new("Held".to_string(), 100, 10, 10, 1),
        Monster::new("Ugly Bastard".to_string(), 50, 10, 10, 10),
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
            if combat.is_fin() {
                break;
            } else {
                match key.code {
                    KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('k') => combat.player.up(),
                    KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j') => combat.player.down(),
                    KeyCode::Esc => break,
                    KeyCode::Enter => match combat.player.get_action() {
                        Action::Hit => combat.hit_monster(),
                        Action::Defense => {
                            // it needs to realize step system player and monster
                            // if this action is pressed, in next step if a monster is attacking, player will get less damage
                            // but right now this will not work
                        }
                        Action::Info => {
                            // make something in draw
                            // show player and monster stats
                        }
                        Action::Run => break,
                        _ => {}
                    },
                    KeyCode::Tab => {
                        // make same as Action::Info
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
