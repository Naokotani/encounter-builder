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
