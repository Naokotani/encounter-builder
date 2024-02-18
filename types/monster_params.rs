use crate::types::state::{EitherBool, FillStatus, Weight};

#[derive(Debug)]
pub struct MonsterParams {
    pub budget: f32,
    pub upper_level: i32,
    pub lower_level: i32,
    pub level: Option<i32>,
    pub status: FillStatus,
    pub number: i32,
    pub is_caster: EitherBool,
    pub is_ranged: EitherBool,
    pub is_aquatic: bool,
    pub weight: Weight,
}

impl MonsterParams {
    pub fn new(is_caster: EitherBool,
               is_ranged: EitherBool,
               is_aquatic: bool,
               weight: Weight,
    ) -> Self {
        let status = match weight {
            Weight::None => FillStatus::Skipped,
            _ => FillStatus::Pending,
        };

        MonsterParams {
            budget: 0.0,
            upper_level: 0,
            lower_level: 0,
            level: None,
            status,
            number: 0,
            is_caster,
            is_ranged,
            is_aquatic,
            weight,
        }
    }
}
