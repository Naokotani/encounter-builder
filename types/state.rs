use serde::Deserialize;
use crate::types::error::EnumError;
use std::fmt;

#[derive(Debug, Deserialize)]
pub enum EncounterBudget {
    Trivial,
    Low,
    Moderate,
    Severe,
    Extreme,
}

impl EncounterBudget {
    pub fn new(string: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let result =match string {
            "trivial" => EncounterBudget::Trivial,
            "low" => EncounterBudget::Low,
            "moderate" => EncounterBudget::Moderate,
            "severe" => EncounterBudget::Severe,
            "extreme" => EncounterBudget::Extreme,
            _ => {
                return Err(Box::new(EnumError::Difficulty(String::from(
                    string
                ))))
            }
        };
        Ok(result)
    }
}

#[derive(Debug, PartialEq)]
pub enum Weight {
    Less,
    Even,
    More,
    All,
    None,
}

impl Weight {
    pub fn new(weight: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_weight = match weight {
            "less" => Weight::Less,
            "even" => Weight::Even,
            "more" => Weight::More,
            "all" => Weight::All,
            "none" => Weight::None,
            _ => return Err(Box::new(EnumError::ConfigWeight(String::from(weight)))),
        };
        Ok(config_weight)
    }

    pub fn get_budget(&self, budget: f32) -> f32 {
        match self {
        Weight::Less => {
            budget * 0.25
        }
        Weight::Even => {
            budget * 0.5
        }
        Weight::More => {
            budget * 0.75
        }
        Weight::All => {
            budget
        }
        Weight::None => {
            0.0
        }
        }
    }
}

#[derive(Debug)]
pub enum EitherBool {
    True,
    False,
    Either,
}

impl EitherBool {
    pub fn new(bool_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let either_bool = match bool_str {
            "either" => EitherBool::Either,
            "true" => EitherBool::True,
            "false" => EitherBool::False,
            _ => return Err(Box::new(EnumError::EitherBool(String::from(bool_str)))),
        };
        Ok(either_bool)
    }
}

impl fmt::Display for EitherBool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           EitherBool::Either => write!(f, "Either"),
           EitherBool::True => write!(f, "True"),
           EitherBool::False => write!(f, "False"),
       }
    }
}

#[derive(PartialEq, Debug)]
pub enum FillStatus {
    Pending,
    Filled,
    Failed,
    Skipped,
}

impl FillStatus {
    pub fn stringify(&self) -> String {
        match self {
            FillStatus::Filled => String::from("Filled"),
            FillStatus::Pending => String::from("Pending"),
            FillStatus::Failed => String::from("Failed"),
            FillStatus::Skipped => String::from("Skipped"),
        }
    }
}


#[cfg(test)]
mod tests {
    use test_log;
    use super::*;

    #[test_log::test]
    pub fn weight() {
        let less = Weight::Less.get_budget(100.0);
        let even = Weight::Even.get_budget(100.0);
        let more = Weight::More.get_budget(100.0);
        let all = Weight::All.get_budget(100.0);
        let none = Weight::None.get_budget(100.0);

        assert_eq!(less, 25.0);
        assert_eq!(even, 50.0);
        assert_eq!(more, 75.0);
        assert_eq!(all, 100.0);
        assert_eq!(none, 0.0);
    }
}
    
