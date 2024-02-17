use crate::handlers::query;
<<<<<<< HEAD
use sqlx::PgPool;
use crate::types::error::SearchError;
=======
>>>>>>> 3a2c391 (improved logging)
use tracing::{event, Level};
use crate::types::state::{EitherBool, EncounterBudget, FillStatus, Weight};
use crate::types::{encounter_params, monster, monster_params};
use rand::Rng;
use sqlx::postgres::PgPoolOptions;
use std::fmt;

#[derive(Debug)]
pub struct Encounter {
    pub level: i32,
    pub party_size: i32,
    pub difficulty: EncounterBudget,
    pub monster_types: Vec<String>,
    pub traits: Vec<String>,
    pub monster_weights: MonsterWeights,
    pub budget: f32,
    pub bbeg: monster_params::MonsterParams,
    pub hench: monster_params::MonsterParams,
    pub lackey: monster_params::MonsterParams,
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
pub struct MonsterWeights {
    pub bbeg: Weight,
    pub henchman: Weight,
    pub lackey: Weight,
}

impl Encounter {
    pub fn new(
        encounter: encounter_params::EncounterParams,
        bbeg: monster_params::MonsterParams,
        hench: monster_params::MonsterParams,
        lackey: monster_params::MonsterParams,
    ) -> Self {
        Encounter {
            level: encounter.level,
            party_size: encounter.party_size,
            difficulty: encounter.difficulty,
            monster_types: encounter.monster_types,
            traits: Vec::new(),
            monster_weights: encounter.monster_weights,
            budget: 0.0,
            bbeg,
            hench,
            lackey,
        }
    }
    pub async fn create(&mut self) -> Result<Vec<monster::Monster>, Box<dyn std::error::Error>> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&std::env::var("DATABASE_URL").expect("Env var didn't load"))
            .await?;

        self.budget = match self.difficulty {
            EncounterBudget::Trivial => 40.0 + adjust_budget(self.party_size),
            EncounterBudget::Low => 60.0 + adjust_budget(self.party_size),
            EncounterBudget::Moderate => 80.0 + adjust_budget(self.party_size),
            EncounterBudget::Severe => 120.0 + adjust_budget(self.party_size),
            EncounterBudget::Extreme => 160.0 + adjust_budget(self.party_size),
        };

<<<<<<< HEAD
        event!(Level::INFO, "Starting budget: {}", self.budget);
=======
        event!(Level::DEBUG, "Starting budget: {}", self.budget);

>>>>>>> 3a2c391 (improved logging)
        let mut monster_list: Vec<monster::Monster> = Vec::new();
        self.get_bbeg_params();

        if self.bbeg.status != FillStatus::Skipped {
<<<<<<< HEAD
        }

        let mut bbeg_attempts = 0;
        if self.bbeg.status == FillStatus::Failed {
            self.bbeg_fail_query(4, &pool ).await?;
            bbeg_attempts += 1;
        }


=======
           let result = query::query(
                &self.monster_types,
                &self.bbeg,
                self.bbeg.level.unwrap(),
               &pool,
                None,
            )
            .await;
            match result {
                Ok(m) => {
                    if let Some(m) = m {
                        event!(Level::DEBUG, "Bbeg budget {:?}, remaining budget: {}", self.bbeg.budget, self.budget);
                        self.bbeg.number = 1;
                        self.bbeg.status = FillStatus::Filled;
                        monster_list.push(m);
                    } else {

                        event!(Level::WARN,
                               "Bbeg failed for level {}, attempting to fill.",
                               self.bbeg.level.unwrap()
                        );
                        self.bbeg.status = FillStatus::Failed;
                        self.bbeg.level = self.bbeg_fail_budget();
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        let mut bbeg_attempts = 0;
        while self.bbeg.status == FillStatus::Failed && bbeg_attempts < 4 {
            event!(Level::WARN,
                   "Bbeg failed for level {}, attempting to fill.",
                   self.bbeg.level.unwrap()
            );
            self.get_bbeg_params();
            if let Some(m) = query::query(
                &self.monster_types,
                &self.bbeg,
                self.bbeg.level.unwrap(),
                &pool,
                None,
            )
            .await?
            {
                event!(Level::DEBUG, "Bbeg budget {:?}, remaining budget: {}", self.bbeg.budget, self.budget);
                monster_list.push(m);
                self.bbeg.number = 1;
                self.bbeg.status = FillStatus::Filled;
            } else {
                self.bbeg.level = self.bbeg_fail_budget();
            };
            bbeg_attempts += 1;
        }

>>>>>>> 3a2c391 (improved logging)
        self.budget -= self.bbeg.budget;

        self.get_hench_params();
        if self.hench.number > 0 && self.hench.status != FillStatus::Skipped {
            let result = query::query(
                &self.monster_types,
                &self.hench,
                self.hench.level.unwrap(),
                &pool,
                None,
            )
            .await;
            match result {
                Ok(m) => {
                    if let Some(m) = m {
                        event!(Level::DEBUG, "Hench budget {:?}, remaining budget: {}", self.hench.budget, self.budget);
                        self.hench.status = FillStatus::Filled;
                        monster_list.push(m);
                    } else {
                        self.hench.status = FillStatus::Failed;
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        } else {
            self.hench.status = FillStatus::Skipped;
        }

        let mut hench_attempts = 0;
        while self.hench.status == FillStatus::Failed
            && hench_attempts < 3
            && self.hench.status != FillStatus::Skipped
        {
            event!(Level::WARN,
                "Hench failed for level {}, attempting to fill.",
                self.hench.level.unwrap()
            );
            self.hench.budget = 0.0;
            self.get_hench_params();
            if let Some(m) = query::query(
                &self.monster_types,
                &self.hench,
                self.hench.level.unwrap(),
                &pool,
                None,
            )
            .await?
            {
                event!(Level::DEBUG, "Hench budget {:?}, remaining budget: {}", self.hench.budget, self.budget);
                monster_list.push(m);
                self.hench.status = FillStatus::Filled;
                break;
            } else if hench_attempts == 2 {
                self.lackey.budget += self.hench.budget;
            };
            if hench_attempts == 2 && self.hench.status == FillStatus::Failed {
                event!(Level::ERROR, "Hench failed to fill, hench budget: {}", self.hench.budget);
            }
            hench_attempts += 1;
        }

        self.budget -= self.hench.budget;

        self.get_lackey_params();
        if self.lackey.number > 0 && self.lackey.status != FillStatus::Skipped {
            let result = query::query(
                &self.monster_types,
                &self.lackey,
                self.lackey.level.unwrap(),
                &pool,
                None,
            )
            .await;

            match result {
                Ok(m) => {
                    if let Some(m) = m {
                        event!(Level::DEBUG, "Lackey budget {:?}, remaining budget: {}", self.lackey.budget, self.budget);
                        self.lackey.status = FillStatus::Filled;
                        monster_list.push(m);
                    } else {
                        self.lackey.status = FillStatus::Failed;
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        if self.lackey.status == FillStatus::Failed {
            event!(Level::WARN,
                "Hench failed for level {}, attempting to fill.",
                self.hench.level.unwrap()
            );
            self.get_lackey_params();
            if let Some(m) = query::query(
                &self.monster_types,
                &self.lackey,
                self.lackey.level.unwrap(),
                &pool,
                None,
            )
            .await?
            {
                event!(Level::DEBUG, "Lackey budget {:?}, remaining budget: {}", self.lackey.budget, self.budget);
                monster_list.push(m);
                self.lackey.status = FillStatus::Filled;
            }
        };

        self.budget -= self.lackey.budget;

        Ok(monster_list)
    }

    fn get_bbeg_params(&mut self) {
        let current_budget = self.budget;
        self.bbeg.level = match self.monster_weights.bbeg {
            Weight::Less => {
                self.bbeg.budget = current_budget * 0.25;
                self.bbeg_budget()
            }
            Weight::Even => {
                self.bbeg.budget = current_budget * 0.5;
                self.bbeg_budget()
            }
            Weight::More => {
                self.bbeg.budget = current_budget * 0.75;
                self.bbeg_budget()
            }
            Weight::All => {
                self.bbeg.budget = current_budget;
                self.bbeg_budget()
            }
            Weight::None => {
                self.bbeg.status = FillStatus::Skipped;
                None
            }
        };
    }

    fn get_hench_params(&mut self) {
        let current_budget = self.budget;
        let henchman_number = match self.monster_weights.henchman {
            Weight::Less => {
                self.hench.budget = current_budget * 0.25;
                self.henchman_budget()
            }
            Weight::Even => {
                self.hench.budget = current_budget * 0.5;
                self.henchman_budget()
            }
            Weight::More => {
                self.hench.budget = current_budget * 0.75;
                self.henchman_budget()
            }
            Weight::All => {
                self.hench.budget = current_budget;
                self.henchman_budget()
            }
            Weight::None => {
                self.hench.status = FillStatus::Skipped;
                0
            }
        };

        self.hench.number = henchman_number;
    }

    fn get_lackey_params(&mut self) {
        let (lackey_number, lackey_level) =
            if self.budget >= 10.0 && self.monster_weights.lackey != Weight::None {
                self.lackey.budget = self.budget;
                self.lackey_budget()
            } else {
                self.lackey.status = FillStatus::Skipped;
                (0, 0)
            };

        self.lackey.level = Some(lackey_level);

        self.lackey.number = lackey_number;
    }

    fn lackey_budget(&mut self) -> (i32, i32) {
        let lackey_mod: i32;
        if self.level == 2 {
            lackey_mod = -3;
            self.lackey.level = Some(self.level - 3);
        } else if self.lackey.level.is_none() {
            let mut rng = rand::thread_rng();
            lackey_mod = rng.gen_range(-4..=-3);
        } else if self.lackey.level.unwrap() == self.level - 4 {
            lackey_mod = -3;
            self.lackey.level = Some(self.level - 3);
        } else {
            lackey_mod = -4;
            self.lackey.level = Some(self.level - 4);
        }

        let budget = self.lackey.budget;
        match lackey_mod {
            -4 => {
                self.lackey.budget = budget - (budget % 10.0);
                ((self.lackey.budget / 10.0).round() as i32, self.level - 4)
            }
            -3 => {
                self.lackey.budget = budget - (budget % 15.0);
                ((self.lackey.budget / 15.0).round() as i32, self.level - 3)
            }
            _ => panic!("invalid random range"),
        }
    }

    fn henchman_budget(&mut self) -> i32 {
        let hench_mod: i32;
        let mut rng = rand::thread_rng();
        if self.hench.level.is_none() && self.hench.budget >= 40.0 {
            hench_mod = rng.gen_range(-2..=0);
        } else if self.hench.level.is_none() && self.hench.budget >= 30.0 {
            hench_mod = rng.gen_range(-1..=0);
        } else if self.hench.level.is_none() && self.hench.budget >= 20.0 {
            hench_mod = -2;
        } else if self.hench.level.is_none() {
            self.hench.status = FillStatus::Skipped;
            self.monster_weights.lackey = Weight::All;
            self.budget += self.hench.budget;
            return 0;
        } else if self.hench.level.unwrap() == self.level {
            hench_mod = -1;
            self.hench.level = Some(self.level - 1);
        } else if self.hench.level.unwrap() == self.level - 1 {
            self.hench.level = Some(self.level - 2);
            hench_mod = -2;
        } else {
            self.hench.level = Some(self.level);
            hench_mod = 0;
        }

        let budget = self.hench.budget;
        match hench_mod {
            -2 => {
                self.hench.level = Some(self.level - 2);
                self.hench.budget = budget - (budget % 20.0);
                (self.hench.budget / 20.0).round() as i32
            }
            -1 => {
                self.hench.level = Some(self.level - 1);
                self.hench.budget = budget - (budget % 30.0);
                (self.hench.budget / 30.0).round() as i32
            }
            0 => {
                self.hench.level = Some(self.level);
                self.hench.budget = budget - (budget % 40.0);
                (self.hench.budget / 40.0).round() as i32
            }
            _ => panic!("invalid random range"),
        }
    }

    fn bbeg_budget(&mut self) -> Option<i32> {
        match self.bbeg.budget {
            b if b >= 160.0 => {
                self.bbeg.budget = 160.0;
                Some(self.level + 4)
            }
            b if b >= 120.0 => {
                self.bbeg.budget = 120.0;
                Some(self.level + 3)
            }
            b if b >= 80.0 => {
                self.bbeg.budget = 80.0;
                Some(self.level + 2)
            }
            b if b >= 60.0 => {
                self.bbeg.budget = 60.0;
                Some(self.level + 1)
            }
            b if b >= 40.0 => {
                self.bbeg.budget = 40.0;
                Some(self.level)
            }
            b if b >= 30.0 => {
                self.bbeg.budget = 30.0;
                Some(self.level - 1)
            }
            b if b >= 20.0 => {
                self.bbeg.budget = 20.0;
                Some(self.level - 2)
            }
            b if b >= 15.0 => {
                self.bbeg.budget = 15.0;
                Some(self.level - 3)
            }
            b if b >= 10.0 => {
                self.bbeg.budget = 10.0;
                Some(self.level - 4)
            }
            _ => None,
        }
    }

    fn bbeg_fail_budget(&mut self) -> Option<i32> {
        match self.level {
            l if l == self.level + 4 => {
                self.bbeg.budget -= 40.0;
                Some(self.level + 3)
            }
            l if l == self.level + 3 => {
                self.bbeg.budget -= 40.0;
                Some(self.level + 2)
            }
            l if l == self.level + 2 => {
                self.bbeg.budget -= 20.0;
                Some(self.level + 1)
            }
            l if l == self.level + 1 => {
                self.bbeg.budget -= 20.0;
                Some(self.level)
            }
            l if l == self.level => {
                self.bbeg.budget -= 10.0;
                Some(self.level - 1)
            }
            l if l == self.level - 1 => {
                self.bbeg.budget -= 10.0;
                Some(self.level - 2)
            }
            l if l == self.level - 2 => {
                self.bbeg.budget -= 10.0;
                Some(self.level - 3)
            }
            l if l == self.level - 3 => {
                self.bbeg.budget -= 5.0;
                Some(self.level - 4)
            }
            l if l == self.level - 4 => Some(self.level - 4),
            _ => None,
        }
    }
<<<<<<< HEAD

async fn bbeg_fail_query(&self, attempts: usize, pool: &PgPool) -> Result<monster::Monster, Box<dyn std::error::Error>> {
    let i: usize = 0;
    let mut m;
    while i < attempts {
        
    event!(Level::WARN,
           "BBEG failed for level {}, attempting to fill.",
           self.bbeg.level.unwrap()
    );
    if let Some(m) = query::query(
        &self.monster_types,
        &self.bbeg,
        self.bbeg.level.unwrap(),
        &pool,
        None,
    )
        .await?
    {
        event!(Level::DEBUG, "BBEG budget {:?}, remaining budget: {}", self.bbeg.budget, self.budget);
        self.bbeg.number = 1;
        self.bbeg.status = FillStatus::Filled;
        m = m;
    } else {
        self.bbeg.level = self.bbeg_fail_budget();
    };
    if attempts == 4 && self.bbeg.status == FillStatus::Failed {
        event!(Level::ERROR, "BBEG failed to fill, bbeg budget: {}", self.bbeg.budget);
    };
        attempts += 1;
    }
    Err(Box::new(EnumError::EitherBool(String::from(bool_str))))
}

async fn bbeg_query() {

    let result = query::query(
        &self.monster_types,
        &self.bbeg,
        self.bbeg.level.unwrap(),
        &pool,
        None,
    )
        .await;
    match result {
        Ok(m) => {
            if let Some(m) = m {
                event!(Level::DEBUG, "BBEG budget {:?}, remaining budget: {}", self.bbeg.budget, self.budget);
                self.bbeg.number = 1;
                self.bbeg.status = FillStatus::Filled;
                monster_list.push(m);
            } else {
                self.bbeg.status = FillStatus::Failed;
                self.bbeg.level = self.bbeg_fail_budget();
            }
        }
        Err(e) => {
            return Err(e);
        }
    }
    
}
=======
>>>>>>> 3a2c391 (improved logging)
}

fn adjust_budget(party_size: i32) -> f32 {
    match party_size {
        1 => -20.0,
        2 => -15.0,
        3 => -10.0,
        4 => 0.0,
        5 => 10.0,
        6 => 15.0,
        7 => 20.0,
        8 => 30.0,
        9 => 40.0,
        _ => panic!("Party size out of range {}", party_size),
    }
}


impl fmt::Display for MonsterWeights {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "
            bbeg: {:?}
            henchman: {:?}
            lackey: {:?}",
            self.bbeg, self.henchman, self.lackey,
        )
    }
}
