use crate::entity::{Monster, Player, Entity, New, Fight};

pub struct Combat {
    pub text: String,
    pub player: Player,
    pub monster: Monster,
}
