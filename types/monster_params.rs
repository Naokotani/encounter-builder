use crate::types::state::{EitherBool, FillStatus};

#[derive(Debug)]
pub struct MonsterParams {
    pub budget: f32,
    pub level: Option<i32>,
    pub status: FillStatus,
    pub number: i32,
    pub is_caster: EitherBool,
    pub is_ranged: EitherBool,
    pub is_aquatic: bool,
}

impl MonsterParams {
    pub fn new(is_caster: EitherBool, is_ranged: EitherBool, is_aquatic: bool) -> Self {
        MonsterParams {
            budget: 0.0,
            level: None,
            status: FillStatus::Pending,
            number: 0,
            is_caster,
            is_ranged,
            is_aquatic,
        }
    }
}
