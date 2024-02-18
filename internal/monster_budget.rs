#[derive(Debug)]
pub struct MonsterBudget {
    pub level: i32,
    pub budget: i32,
    pub number: i32,
}

impl MonsterBudget {
    pub fn bbeg_budget(level: i32, budget: i32) -> Self {
        MonsterBudget {
            level,
            budget,
            number: 1,
        }
    }

    pub fn hench_budget(party_level: i32, level: i32, budget: i32) -> Self {
        let level_mod = level - party_level;
        let hench_mod = match level_mod {
            -2 if budget >= 30 => -1,
            -1 if budget >= 40 => 0,
            0 if party_level > 1 => -2,
            _ if budget >= 30 => -1,
            -2 => -2,
            _ => panic!("Level mod out of range {}", level),
        };

        let mut budget = budget as f32;

        match hench_mod {
            -2 => {
                budget -= budget % 20.0;
                let number = (budget / 20.0).round() as i32;
                MonsterBudget {
                    budget: number * 20,
                    level: party_level - 2,
                    number,
                }
            }
            -1 => {
                budget -= budget % 30.0;
                let number = (budget / 30.0).round() as i32;
                MonsterBudget {
                    budget: number * 30,
                    level: party_level - 1,
                    number,
                }
            }
            0 => {
                budget -= budget % 40.0;
                let number = (budget / 40.0).round() as i32;
                MonsterBudget {
                    budget: number * 40,
                    level: party_level,
                    number,
                }
            }
            _ => panic!("invalid random range"),
        }
    }

    pub fn lackey_budget(party_level: i32, level: i32, budget: i32) -> Self {
        println!("Level: {}, Party Level: {}", level, party_level);
        let level_mod = level - party_level;
        let lackey_mod = match level_mod {
            -3 if party_level > 3 && budget >= 20 => -4,
            -4 => -3,
            -3 => -3,
            l => panic!("Level mod out of range {}", l),
        };

        let mut budget = budget as f32;
        match lackey_mod {
            -4 => {
                budget -= budget % 10.0;
                let number = (budget / 10.0).round() as i32;
                MonsterBudget {
                    budget: number * 10,
                    level: party_level - 4,
                    number,
                }
            }
            -3 => {
                budget -= budget % 15.0;
                let number = (budget / 15.0).round() as i32;
                MonsterBudget {
                    budget: number * 15,
                    level: party_level - 3,
                    number,
                }
            }
            _ => panic!("invalid random range"),
        }
    }
}
