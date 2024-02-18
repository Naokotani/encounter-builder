use crate::types::state::EncounterBudget;

pub struct EncounterParams {
    pub level: i32,
    pub party_size: i32,
    pub difficulty: EncounterBudget,
    pub monster_types: Vec<String>,
    pub traits: Vec<String>,
    pub budget: f32,
}

impl EncounterParams {
    pub fn new(
        level: i32,
        party_size: i32,
        difficulty: EncounterBudget,
        monster_types: Vec<String>,
    ) -> Self {
        EncounterParams {
            level,
            party_size,
            difficulty,
            monster_types,
            traits: Vec::new(),
            budget: 0.0,
        }
    }
}
