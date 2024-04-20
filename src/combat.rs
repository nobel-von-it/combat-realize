use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Gauge, Paragraph};
use crate::entity::{Monster, Player, New, Fight};

const PL_DISPL: &str = r#"
           O
         [-|-!
           ||
"#;
const TMP_MONSTER: &str = r#"
     00
     ^
   ==[]==
    =[]=
     ||
"#;
const WIN: &str = r#"
 __      ___      _ _
 \ \    / (_)_ _ | | |
  \ \/\/ /| | ' \|_|_|
   \_/\_/ |_|_||_(_|_)
"#;
const LOSE: &str = r#"
  ___  _        _
 |   \(_)___ __| |
 | |) | / -_) _` |_ _
 |___/|_\___\__,_(_|_)
"#;


pub struct Combat {
    pub text: String,
    pub player: Player,
    pub monster: Monster,
}
impl Combat {
    pub fn new(player: Player, monster: Monster) -> Self {
        Self {
            text: String::new(),
            player,
            monster,
        }
    }
    pub fn hit_monster(&mut self) {
        self.monster.entity.get_damage(self.player.entity.damage)
    }
    pub fn hit_player(&mut self) {
        self.player.entity.get_damage(self.monster.entity.damage)
    }
    pub fn is_fin(&self) -> bool {
        self.player.entity.now_hp == 0 || self.monster.entity.now_hp == 0
    }
    pub fn draw(&self, f: &mut Frame) {
        /* Widgets */

        // hp
        let player_hp = Gauge::default()
            .percent(self.player.entity.get_percent_hp());
        let monster_hp = Gauge::default()
            .percent(self.monster.entity.get_percent_hp());

        // upper text that describe action
        let action_describe = Paragraph::new(self.text.clone())
            .block(Block::new().borders(Borders::ALL).title("Fighting"));

        // display entity who fighting now
        let player_display = Paragraph::new(PL_DISPL).centered()
            .block(Block::default().borders(Borders::RIGHT));
        let monster_display = Text::raw(TMP_MONSTER);


        // names
        let player_name = Paragraph::new(self.player.entity.name.clone()).centered();
        let monster_name = Paragraph::new(self.monster.entity.name.clone()).centered();

        // Layouts
        let full_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(90),
            ]).split(f.size());
        let enemy_display_layout = half_rect(Direction::Horizontal).split(full_layout[1]);

        let player_display_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Percentage(50), Constraint::Length(1), Constraint::Percentage(50)])
            .split(enemy_display_layout[0]);
        let monster_display_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Percentage(50), Constraint::Length(1), Constraint::Percentage(50)])
            .split(enemy_display_layout[1]);

        /* Renders */
        f.render_widget(action_describe, full_layout[0]);

        // Player
        f.render_widget(player_name, player_display_layout[0]);
        f.render_widget(player_display, player_display_layout[1]);
        f.render_widget(player_hp, player_display_layout[2]);

        // Monster
        f.render_widget(monster_name, monster_display_layout[0]);
        f.render_widget(monster_display, monster_display_layout[1]);
        f.render_widget(monster_hp, monster_display_layout[2]);

        /* Check win/lose */
        if self.player.entity.now_hp == 0 {
            let lose_display = Paragraph::new(LOSE).centered();
            f.render_widget(lose_display, centered_rect(30, 30, f.size()))
        }
        if self.monster.entity.now_hp == 0 {
            let win_display = Paragraph::new(WIN).centered();
            f.render_widget(win_display, centered_rect(30, 30, f.size()))
        }
    }
}
fn half_rect(direction: Direction) -> Layout {
    Layout::default()
        .direction(direction)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ])
}
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = half_centered_rect(percent_y, Direction::Vertical)
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    half_centered_rect(percent_x, Direction::Horizontal)
        .split(popup_layout[1])[1]
}
fn half_centered_rect(percent: u16, direction: Direction) -> Layout {
    Layout::default()
        .direction(direction)
        .constraints([
            Constraint::Percentage((100 - percent) / 2),
            Constraint::Percentage(percent),
            Constraint::Percentage((100 - percent) / 2),
        ])
}