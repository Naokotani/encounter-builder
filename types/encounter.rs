use crate::types::monster;
use std::fmt;
use crate::handlers::query;
use rand::Rng;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;

#[derive(Debug)]
pub struct Encounter {
    pub level: i32,
    pub party_size: i32,
    pub difficulty: EncounterBudget,
    pub monster_types: Vec<String>,
    pub traits: Vec<String>,
    pub configuration: Configuration,
    pub budget: f32,
    pub bbeg_budget: f32,
    pub bbeg_level: Option<i32>,
    pub hench_budget: f32,
    pub hench_level: Option<i32>,
    pub lackey_budget: f32,
    pub lackey_level: Option<i32>,
    pub bbeg_status: FillStatus,
    pub bbeg_caster: EitherBool,
    pub bbeg_ranged: EitherBool,
    pub bbeg_aquatic: bool,
    pub hench_status: FillStatus,
    pub hench_caster: EitherBool,
    pub hench_ranged: EitherBool,
    pub hench_aquatic: bool,
    pub lackey_status: FillStatus,
    pub lackey_caster: EitherBool,
    pub lackey_ranged: EitherBool,
    pub lackey_aquatic: bool,
}


#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub enum EncounterBudget {
    Trivial,
    Low,
    Moderate,
    Severe,
    Extreme,
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum ConfigWeight {
    Less,
    Even,
    More,
    All,
    None,
}

#[derive(Debug)]
pub enum EitherBool {
    True,
    False,
    Either,
}

impl Clone for EitherBool {
    fn clone(&self) -> EitherBool {
        match self {
            EitherBool::True => EitherBool::True,
            EitherBool::False => EitherBool::False,
            EitherBool::Either => EitherBool::Either,
        }
    }
}

#[derive(Debug)]
pub struct Configuration {
    pub bbeg: ConfigWeight,
    pub henchman: ConfigWeight,
    pub lackey: ConfigWeight,
}


#[derive(PartialEq, Debug)]
pub enum FillStatus {
    Pending,
    Filled,
    Failed,
    Skipped,
}

#[derive(Debug)]
pub struct MonsterGroup {
    pub number: i32,
    pub is_ranged: EitherBool,
    pub is_caster: EitherBool,
    pub is_aquatic: bool,
}

impl Encounter {
    pub async fn create(&mut self) -> Result<Vec<monster::Monster>, Box<dyn std::error::Error>> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&std::env::var("DATABASE_URL").expect("Env var didn't load"))
            .await?;

        self.budget = match self.difficulty {
            EncounterBudget::Trivial => 40.0 + self.adjust_budget(),
            EncounterBudget::Low => 60.0 + self.adjust_budget(),
            EncounterBudget::Moderate => 80.0 + self.adjust_budget(),
            EncounterBudget::Severe => 120.0 + self.adjust_budget(),
            EncounterBudget::Extreme => 160.0 + self.adjust_budget(),
        };

        println!("\nStarting budget: {}", self.budget);

        let mut monster_list: Vec<monster::Monster> = Vec::new();
        let mut bbeg_params = self.get_bbeg_params();
        if self.bbeg_status != FillStatus::Skipped {
            let result = query::query(
                &self.monster_types,
                &bbeg_params,
                self.bbeg_level.unwrap(),
                &pool,
                None,
            )
            .await;
            match result {
                Ok(m) => {
                    if let Some(m) = m {
                        self.bbeg_status = FillStatus::Filled;
                        monster_list.push(m);
                    } else {
                        self.bbeg_status = FillStatus::Failed;
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        let mut bbeg_attempts = 0;
        while self.bbeg_status == FillStatus::Failed && bbeg_attempts < 4 {
            println!(
                "BBEG failed for level {}, attempting to fill.",
                self.bbeg_level.unwrap()
            );
            bbeg_params = self.get_bbeg_params();
            if let Some(m) = query::query(
                &self.monster_types,
                &bbeg_params,
                self.bbeg_level.unwrap(),
                &pool,
                None,
            )
            .await?
            {
                self.bbeg_budget();
                println!("BBEG filled successfully");
                monster_list.push(m);
                self.bbeg_status = FillStatus::Filled;
            } else if bbeg_attempts == 3 {
            };
            bbeg_attempts += 1;
        }

        self.budget -= self.bbeg_budget;

        println!("\nBBEG monster group: {:?}", bbeg_params);
        println!("BBEG budget {}", self.bbeg_budget);
        println!("Remaining budget {}\n", self.budget);

        let mut hench_params = self.get_hench_params();
        if hench_params.number > 0 && self.hench_status != FillStatus::Skipped {
            let result = query::query(
                &self.monster_types,
                &hench_params,
                self.hench_level.unwrap(),
                &pool,
                None
            )
            .await;
            match result {
                Ok(m) => {
                    if let Some(m) = m {
                        self.hench_status = FillStatus::Filled;
                        monster_list.push(m);
                    } else {
                        self.hench_status = FillStatus::Failed;
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        } else {
            self.hench_status = FillStatus::Skipped;
        }

        let mut hench_attempts = 0;
        while self.hench_status == FillStatus::Failed
            && hench_attempts < 3
            && self.hench_status != FillStatus::Skipped
        {
            println!(
                "Henchman failed for level {}, attempting to fill.",
                self.hench_level.unwrap()
            );
            self.hench_budget = 0.0;
            hench_params = self.get_hench_params();
            if let Some(m) = query::query(
                &self.monster_types,
                &hench_params,
                self.hench_level.unwrap(),
                &pool,
                None
            )
            .await?
            {
                println!("Henchman filled successfully");
                monster_list.push(m);
                self.hench_status = FillStatus::Filled;
                break;
            } else if hench_attempts == 2 {
                self.lackey_budget += self.hench_budget;
            };
            hench_attempts += 1;
        }

        self.budget -= self.hench_budget;

        println!("\nHench monster group: {:?}", hench_params);
        println!("Hench budget {}", self.hench_budget);
        println!("Remaining budget {}", self.budget);

        let mut lackey_params = self.get_lackey_params();
        if lackey_params.number > 0 && self.lackey_status != FillStatus::Skipped {
            let result = query::query(
                &self.monster_types,
                &lackey_params,
                self.lackey_level.unwrap(),
                &pool,
                None,
            )
            .await;

            match result {
                Ok(m) => {
                    if let Some(m) = m {
                        self.lackey_status = FillStatus::Filled;
                        monster_list.push(m);
                    } else {
                        self.lackey_status = FillStatus::Failed;
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        if self.lackey_status == FillStatus::Failed {
            println!(
                "Lackey failed for level {}, attempting to fill.",
                self.lackey_level.unwrap()
            );
            lackey_params = self.get_lackey_params();
            if let Some(m) = query::query(
                &self.monster_types,
                &lackey_params,
                self.lackey_level.unwrap(),
                &pool,
                None,
            )
            .await?
            {
                println!("Lackey filled successfully");
                monster_list.push(m);
                self.lackey_status = FillStatus::Filled;
            }
        };

        self.budget -= self.lackey_budget;

        println!("\nLackey monster group: {:?}", lackey_params);
        println!("lackey budget {}", self.lackey_budget);
        println!("Remaining budget {}", self.budget);

        Ok(monster_list)
    }

    fn adjust_budget(&self) -> f32 {
        match self.party_size {
            1 => -20.0,
            2 => -15.0,
            3 => -10.0,
            4 => 0.0,
            5 => 10.0,
            6 => 15.0,
            7 => 20.0,
            8 => 30.0,
            9 => 40.0,
            _ => panic!("Party size out of range {}", self.party_size),
        }
    }

    fn get_bbeg_params(&mut self) -> MonsterGroup {
        let current_budget = self.budget;
        self.bbeg_level = match self.configuration.bbeg {
            ConfigWeight::Less => {
                self.bbeg_budget = current_budget * 0.25;
                self.bbeg_budget()
            }
            ConfigWeight::Even => {
                self.bbeg_budget = current_budget * 0.5;
                self.bbeg_budget()
            }
            ConfigWeight::More => {
                self.bbeg_budget = current_budget * 0.75;
                self.bbeg_budget()
            }
            ConfigWeight::All => {
                self.bbeg_budget = current_budget;
                self.bbeg_budget()
            }
            ConfigWeight::None => {
                self.bbeg_status = FillStatus::Skipped;
                None
            }
        };

        MonsterGroup {
            number: 1,
            is_caster: self.bbeg_caster.clone(),
            is_ranged: self.bbeg_ranged.clone(),
            is_aquatic: self.bbeg_aquatic,
        }
    }

    fn get_hench_params(&mut self) -> MonsterGroup {
        let current_budget = self.budget;
        let henchman_number = match self.configuration.henchman {
            ConfigWeight::Less => {
                self.hench_budget = current_budget * 0.25;
                self.henchman_budget()
            }
            ConfigWeight::Even => {
                self.hench_budget = current_budget * 0.5;
                self.henchman_budget()
            }
            ConfigWeight::More => {
                self.hench_budget = current_budget * 0.75;
                self.henchman_budget()
            }
            ConfigWeight::All => {
                self.hench_budget = current_budget;
                self.henchman_budget()
            }
            ConfigWeight::None => {
                self.hench_status = FillStatus::Skipped;
                0
            }
        };

        MonsterGroup {
            number: henchman_number,
            is_ranged: self.hench_ranged.clone(),
            is_caster: self.hench_caster.clone(),
            is_aquatic: self.hench_aquatic,
        }
    }

    fn get_lackey_params(&mut self) -> MonsterGroup {
        let (lackey_number, lackey_level) =
            if self.budget >= 10.0 && self.configuration.lackey != ConfigWeight::None {
                self.lackey_budget = self.budget;
                self.lackey_budget()
            } else {
                self.lackey_status = FillStatus::Skipped;
                (0, 0)
            };

        self.lackey_level = Some(lackey_level);
        MonsterGroup {
            number: lackey_number,
            is_ranged: self.lackey_ranged.clone(),
            is_caster: self.lackey_caster.clone(),
            is_aquatic: self.lackey_aquatic,
        }
    }

    fn lackey_budget(&mut self) -> (i32, i32) {
        let lackey_mod: i32;
        if self.level == 2 {
            lackey_mod = -3;
            self.lackey_level = Some(self.level - 3);
        } else if self.lackey_level.is_none() {
            let mut rng = rand::thread_rng();
            lackey_mod = rng.gen_range(-4..=-3);
        } else if self.lackey_level.unwrap() == self.level - 4 {
            lackey_mod = -3;
            self.lackey_level = Some(self.level - 3);
        } else {
            lackey_mod = -4;
            self.lackey_level = Some(self.level - 4);
        }

        let budget = self.lackey_budget;
        match lackey_mod {
            -4 => {
                self.lackey_budget = budget - (budget % 10.0);
                ((self.lackey_budget / 10.0).round() as i32, self.level - 4)
            }
            -3 => {
                self.lackey_budget = budget - (budget % 15.0);
                ((self.lackey_budget / 15.0).round() as i32, self.level - 3)
            }
            _ => panic!("invalid random range"),
        }
    }

    fn henchman_budget(&mut self) -> i32 {
        let hench_mod: i32;
        let mut rng = rand::thread_rng();
        if self.hench_level.is_none() && self.hench_budget >= 40.0 {
            hench_mod = rng.gen_range(-2..=0);
        } else if self.hench_level.is_none() && self.hench_budget >= 30.0 {
            hench_mod = rng.gen_range(-1..=0);
        } else if self.hench_level.is_none() && self.hench_budget >= 20.0 {
            hench_mod = -2;
        } else if self.hench_level.is_none() {
            self.hench_status = FillStatus::Skipped;
            self.configuration.lackey = ConfigWeight::All;
            self.budget += self.hench_budget;
            return 0;
        } else if self.hench_level.unwrap() == self.level {
            hench_mod = -1;
            self.hench_level = Some(self.level - 1);
        } else if self.hench_level.unwrap() == self.level - 1 {
            self.hench_level = Some(self.level - 2);
            hench_mod = -2;
        } else {
            self.hench_level = Some(self.level);
            hench_mod = 0;
        }

        let budget = self.hench_budget;
        match hench_mod {
            -2 => {
                self.hench_level = Some(self.level - 2);
                self.hench_budget = budget - (budget % 20.0);
                (self.hench_budget / 20.0).round() as i32
            }
            -1 => {
                self.hench_level = Some(self.level - 1);
                self.hench_budget = budget - (budget % 30.0);
                (self.hench_budget / 30.0).round() as i32
            }
            0 => {
                self.hench_level = Some(self.level);
                self.hench_budget = budget - (budget % 40.0);
                (self.hench_budget / 40.0).round() as i32
            }
            _ => panic!("invalid random range"),
        }
    }

    fn bbeg_budget(&mut self) -> Option<i32> {
        if self.bbeg_level.is_none() {
            match self.bbeg_budget {
                b if b >= 160.0 => {
                    self.bbeg_budget = 160.0;
                    Some(self.level + 4)
                }
                b if b >= 120.0 => {
                    self.bbeg_budget = 120.0;
                    Some(self.level + 3)
                }
                b if b >= 80.0 => {
                    self.bbeg_budget = 80.0;
                    Some(self.level + 2)
                }
                b if b >= 60.0 => {
                    self.bbeg_budget = 60.0;
                    Some(self.level + 1)
                }
                b if b >= 40.0 => {
                    self.bbeg_budget = 40.0;
                    Some(self.level)
                }
                b if b >= 30.0 => {
                    self.bbeg_budget = 30.0;
                    Some(self.level - 1)
                }
                b if b >= 20.0 => {
                    self.bbeg_budget = 20.0;
                    Some(self.level - 2)
                }
                b if b >= 15.0 => {
                    self.bbeg_budget = 15.0;
                    Some(self.level - 3)
                }
                b if b >= 10.0 => {
                    self.bbeg_budget = 10.0;
                    Some(self.level - 4)
                }
                _ => None,
            }
        } else {
            let budget = self.bbeg_budget.round() as i32;
            match budget {
                160 => {
                    self.bbeg_budget = 120.0;
                    Some(self.level + 3)
                }
                120 => {
                    self.bbeg_budget = 80.0;
                    Some(self.level + 2)
                }
                80 => {
                    self.bbeg_budget = 60.0;
                    Some(self.level + 1)
                }
                60 => {
                    self.bbeg_budget = 40.0;
                    Some(self.level)
                }
                40 => {
                    self.bbeg_budget = 30.0;
                    Some(self.level - 1)
                }
                30 => {
                    self.bbeg_budget = 20.0;
                    Some(self.level - 2)
                }
                20 => {
                    self.bbeg_budget = 15.0;
                    Some(self.level - 3)
                }
                15 => {
                    self.bbeg_budget = 10.0;
                    Some(self.level - 4)
                }
                _ => None,
            }
        }
    }
}

impl fmt::Display for Encounter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
    level: {} party_size: {} budget: {}
    difficulty: {:?}
    monster_types: {:?}
    traits: {:?}
    configuration: {}
    bbeg_budget: {}
    bbeg_level: {:?}
    hench_budget: {}
    hench_level: {:?}
    lackey_budget: {}
    lackey_level: {:?}
    bbeg_status: {:?} bbeg_caster: {:?} bbeg_ranged: {:?}
    hench_status: {:?} hench_caster: {:?} hench_ranged: {:?}
    lackey_status: {:?} lackey_caster: {:?} lackey_ranged: {:?}\n",
            self.level,
            self.party_size,
            self.budget,
            self.difficulty,
            self.monster_types,
            self.traits,
            self.configuration,
            self.bbeg_budget,
            self.bbeg_level,
            self.hench_budget,
            self.hench_level,
            self.lackey_budget,
            self.lackey_level,
            self.bbeg_status,
            self.bbeg_caster,
            self.bbeg_ranged,
            self.hench_status,
            self.hench_caster,
            self.hench_ranged,
            self.lackey_status,
            self.lackey_caster,
            self.lackey_ranged,
        )
    }
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "
            bbeg: {:?}
            henchman: {:?}
            lackey: {:?}",
            self.bbeg,
            self.henchman,
            self.lackey,
        )
    }
}

