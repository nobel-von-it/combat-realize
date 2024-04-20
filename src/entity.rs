use ratatui::widgets::HighlightSpacing;

pub trait New {
    fn new(name: String, full_hp: u16, damage: u16, armor: u16, dodge: u16) -> Self;
}
pub trait Fight {
    fn get_damage(&mut self, damage: u16);
    fn get_percent_hp(&self) -> u16;
}


const ARMOR_100_PR: u16 = u16::MAX;

pub struct Entity {
    pub name: String,
    pub full_hp: u16,
    pub now_hp: u16,
    pub damage: u16,
    pub armor: u16,
    pub dodge: u16,
}
impl New for Entity {
    fn new(name: String, full_hp: u16, damage: u16, armor: u16, dodge: u16) -> Self {
        Self {
            name,
            full_hp,
            now_hp: full_hp,
            damage,
            armor,
            dodge,
        }
    }
}
impl Fight for Entity {
    fn get_damage(&mut self, damage: u16) {
        if self.now_hp > damage {
            self.now_hp -= damage
        } else {
            self.now_hp = 0
        }
    }
    fn get_percent_hp(&self) -> u16 {
        if self.full_hp == self.now_hp {
            100
        } else {
            self.now_hp * 100 / self.full_hp
        }
    }
}

#[derive(Clone, Debug)]
pub enum Action {
    Hit,
    Defense,
    Run,
}

pub struct Player {
    pub entity: Entity,
    pub actions: Vec<Action>,
    pub select: usize,
}
impl New for Player {
    fn new(name: String, full_hp: u16, damage: u16, armor: u16, dodge: u16) -> Self {
        Self {
            entity: Entity::new(name, full_hp, damage, armor, dodge),
            actions: vec![Action::Hit, Action::Defense, Action::Run],
            select: 0
        }
    }
}
impl Player {
    pub fn up(&mut self) {
        if self.select > 0 {
            self.select -= 1
        }
    }
    pub fn down(&mut self) {
        if self.select < self.actions.len() {
            self.select += 1
        }
    }
    pub fn get_action(&self) -> Action {
        self.actions[self.select].clone()
    }
}
pub struct Monster {
    pub entity: Entity,
}
impl New for Monster {
    fn new(name: String, full_hp: u16, damage: u16, armor: u16, dodge: u16) -> Self {
        Self { entity: Entity::new(name, full_hp, damage, armor, dodge)}
    }
}
impl Monster {
    pub fn ai_step(&mut self) {

    }
}
