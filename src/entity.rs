
pub trait New {
    fn new(name: String, full_hp: u16, damage: u16, armor: u16, dodge: u16) -> Self;
}
pub trait Fight {
    fn get_damage(&mut self, damage: u16);
}


const ARMOR_100_PR: u16 = u16::max_value();

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

pub struct Player {
    pub entity: Entity,
}
impl New for Player {
    fn new(name: String, full_hp: u16, damage: u16, armor: u16, dodge: u16) -> Self {
        Self { entity: Entity::new(name, full_hp, damage, armor, dodge)}
    }
}
impl Fight for Player {
    fn get_damage(&mut self, damage: u16) {
        let real_damage = damage - (damage as f32 / )
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
