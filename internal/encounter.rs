use crate::handlers::query;
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

        event!(Level::INFO, self.level);
        println!("Level {}", self.level);

        let (bbeg_upper_level, bbeg_lower_level) = bbeg_range(self.level);
        let (hench_upper_level, hench_lower_level) = hench_range(self.level);
        let (lackey_upper_level, lackey_lower_level) = lackey_range(self.level);
        
        let (bbeg_list, hench_list, lackey_list) = tokio::join!(
            query::query(&self.monster_types,
                         &self.bbeg,
                         bbeg_upper_level,
                         bbeg_lower_level,
                         &pool,
                         None),
            query::query(&self.monster_types,
                         &self.hench,
                         hench_upper_level,
                         hench_lower_level,
                         &pool,
                         None),
            query::query(&self.monster_types,
                         &self.lackey,
                         lackey_upper_level,
                         lackey_lower_level,
                         &pool,
                         None)
        );

        self.bbeg.weight.get_budget(self.budget);


        let mut monster_list: Vec<monster::Monster>= Vec::new();
        
        if self.bbeg.status == FillStatus::Pending {
            if let Some(m) = bbeg_list.unwrap() {
                self.bbeg.status = FillStatus::Filled;
                monster_list.push(self.bbeg_budget(m, self.bbeg.weight.get_budget(self.budget)));
            } else {
                self.bbeg.status = FillStatus::Failed;
            }
        }

        self.budget -= self.bbeg.budget;
        self.hench.weight.get_budget(self.budget);

        if self.hench.status == FillStatus::Pending {
            if let Some(m) = hench_list.unwrap() {
                self.hench.status = FillStatus::Filled;
                monster_list.push(self.select_hench(m, self.level, self.hench.weight.get_budget(self.budget)));
            } else {
                self.hench.status = FillStatus::Failed;
            }
        }

        self.budget -= self.hench.budget;

        self.lackey.weight.get_budget(self.budget);
        if self.lackey.status == FillStatus::Pending {
            if let Some(m) = lackey_list.unwrap() {
                self.lackey.status = FillStatus::Filled;
                monster_list.push(self.select_lackey(m, self.level, self.lackey.weight.get_budget(self.budget)));
            } else {
                self.lackey.status = FillStatus::Failed;
            }
        }

        self.budget -= self.lackey.budget;

        Ok(monster_list)
    }

    fn bbeg_budget(&mut self, monsters: Vec<monster::MonsterData>, budget: f32) -> monster::Monster {

        let mut range: Vec<i32> = Vec::new();
        for monster in monsters.iter() {
            if !range.contains(&monster.level.unwrap()) {
                range.push(monster.level.unwrap());
            }
        }

        let level = self.level;
        match budget {
            b if b >= 160.0 && range.contains(&(level + 4)) => {
                self.bbeg.budget = 160.0;
                self.bbeg.level = Some(self.level + 4);
            }
            b if b >= 120.0 && range.contains(&(level + 3))=> {
                self.bbeg.budget = 120.0;
                self.bbeg.level = Some(self.level + 3);
            }
            b if b >= 80.0 && range.contains(&(level + 2)) => {
                self.bbeg.budget = 80.0;
                self.bbeg.level = Some(self.level + 2);
            }
            b if b >= 60.0 && range.contains(&(level + 1)) => {
                self.bbeg.budget = 60.0;
                self.bbeg.level = Some(self.level + 1);
            }
            b if b >= 40.0 && range.contains(&level) => {
                self.bbeg.budget = 40.0;
                self.bbeg.level = Some(self.level);
            }
            b if b >= 30.0 && range.contains(&(level - 1)) => {
                self.bbeg.budget = 30.0;
                self.bbeg.level = Some(self.level - 1);
            }
            b if b >= 20.0 && range.contains(&(level - 2)) => {
                self.bbeg.budget = 20.0;
                self.bbeg.level = Some(self.level - 2);
            }
            b if b >= 15.0 && range.contains(&(level - 3)) => {
                self.bbeg.budget = 15.0;
                self.bbeg.level = Some(self.level - 3);
            }
            b if b >= 10.0 && range.contains(&(level -4)) => {
                self.bbeg.budget = 10.0;
                self.bbeg.level = Some(self.level - 4);
            }
            _ => panic!("Bbeg level out of range"),
        }

        let mut candidates: Vec<monster::MonsterData> = Vec::new();
        
        for monster in monsters {
            if monster.level.unwrap() == self.bbeg.level.unwrap() {
                candidates.push(monster);
            }
        }

        let mut rng = rand::thread_rng();

        let index = if candidates.len() > 1 {
           rng.gen_range(0..candidates.len()) 
        } else {
            0
        };
    
        monster::Monster::new(
            candidates.remove(index),
            vec![String::from("Undead")],
            1
        ).unwrap()

    }

    fn select_lackey(&mut self,
                 mut list: Vec<monster::MonsterData>,
                 level: i32, budget: f32) -> monster::Monster {
    let mut rng = rand::thread_rng();

    let index = rng.gen_range(0..list.len());
    
    let lackey = list.remove(index);

    println!("Budget: {}", budget);
    let number = match lackey.level.unwrap() {
        l if l == level -4 => {
            self.lackey.budget = budget - (budget % 10.0);
            (self.lackey.budget / 10.0).round() as i32
        }
        l if l == level -3 => {
            self.lackey.budget = budget - (budget % 15.0);
            (self.lackey.budget / 15.0).round() as i32
        }
        _ => panic!("invalid random range"),
    };

    event!(Level::INFO, number);
    monster::Monster::new(lackey, vec![String::from("Undead")], number).unwrap()
}

fn select_hench(&mut self,
                 mut list: Vec<monster::MonsterData>,
                 level: i32, budget: f32) -> monster::Monster {

    let mut rng = rand::thread_rng();

    let index = rng.gen_range(0..list.len());
    
    let hench = list.remove(index);

    println!("Budget: {}", budget);

    let number = match hench.level.unwrap() {
        l if l == level - 2 => {
            self.hench.budget = budget - (budget % 20.0);
            (self.hench.budget / 20.0).round() as i32
        }
        l if l == level - 1 => {
            self.hench.budget = budget - (budget % 30.0);
            (self.hench.budget/ 30.0).round() as i32
        }
        l if l == level => {
            self.hench.budget = budget - (budget % 40.0);
            (self.hench.budget / 40.0).round() as i32
        }
        _ => panic!("invalid random range"),
    };

    event!(Level::INFO, number);

    monster::Monster::new(hench, vec![String::from("Undead")], number).unwrap()
}
}



fn bbeg_range(level: i32) -> (i32, i32) {
    let upper = level + 4;
    let mut lower = level - 4;
    if lower < -1 {
       lower = -1; 
    }
    (upper, lower)
}

fn hench_range(level: i32) -> (i32, i32) {
    let upper = level;
    let lower =  level - 2;
    (upper, lower)
}

fn lackey_range(level: i32) -> (i32, i32) {
    let mut upper = level - 3;
    let mut lower = level - 4;
    if upper < -1{
        upper = -1;
    }

    if lower < -1 {
        lower = -1;
    }
    (upper, lower)
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
