use rand::Rng;

pub trait New {
    fn new(name: String, full_hp: u16, damage: u16, armor: u16, dodge: u16) -> Self;
}
pub trait Fight {
    fn get_damage(&mut self, damage: u16) -> String;
    fn get_percent_hp(&self) -> u16;
}

// Percentage caps
const ARMOR_CAP: u16 = 50;
const DODGE_CAP: u16 = 20;
const ERROR_RANGE_PERCENTAGE: u16 = 20;

// Default value caps
const DAMAGE_CAP: u16 = u16::MAX / 2;

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
        let real_armor = if armor > ARMOR_CAP { ARMOR_CAP } else { armor };
        let real_dodge = if dodge > DODGE_CAP { DODGE_CAP } else { dodge };
        let real_damage = if damage > DAMAGE_CAP {
            DAMAGE_CAP
        } else {
            damage
        };
        Self {
            name,
            full_hp,
            now_hp: full_hp,
            damage: real_damage,
            armor: real_armor,
            dodge: real_dodge,
        }
    }
}
impl Fight for Entity {
    fn get_damage(&mut self, damage: u16) -> String {
        // add chance error, that every damage was difference
        // if damage is 10, then real_damage until armor calculating will from 8 to 12 (+-2 is error)

        let error_less_damage = damage * (100 - ERROR_RANGE_PERCENTAGE) / 100;
        let error_more_damage = damage * (100 + ERROR_RANGE_PERCENTAGE) / 100;

        let damage_with_errors =
            rand::thread_rng().gen_range(error_less_damage..=error_more_damage);

        let mut real_damage = (100 - self.armor) * damage_with_errors / 100;
        if self.dodge < rand::thread_rng().gen_range(0..=100) as u16 {
            real_damage = if real_damage > DAMAGE_CAP {
                DAMAGE_CAP
            } else {
                real_damage
            };
            if self.now_hp > real_damage {
                self.now_hp -= real_damage
            } else {
                self.now_hp = 0
            }
            if self.now_hp == 0 {
                return format!("{} погибает в муках.", self.name);
            }
            return format!("{} получил {} урона.", self.name, real_damage);
        }
        format!("{} увернулся.", self.name)
    }
    fn get_percent_hp(&self) -> u16 {
        self.now_hp * 100 / self.full_hp
    }
}
#[derive(Clone, Debug)]
pub enum Action {
    Hit,
    Defense,
    Info,
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
            actions: vec![Action::Hit, Action::Defense, Action::Info, Action::Run],
            select: 0,
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
        if self.select < self.actions.len() - 1 {
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
        Self {
            entity: Entity::new(name, full_hp, damage, armor, dodge),
        }
    }
}
impl Monster {
    pub fn get_monster_action(&self, player: &Player) -> Action {
        if self.entity.full_hp * 3 < player.entity.full_hp {
            Action::Run
        } else {
            Action::Hit
        }
    }
}
