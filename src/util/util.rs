use std::str::FromStr;

use google_sheets4::api::ValueRange;
use serde::{Deserialize, Serialize};

/// It determines the status of a project, epic or something like that:
///
/// - `Pending`: When the project/epic has been created but it has not one advancing.
/// - `InProgress`: When an project/epic has been started.
/// - `Done`: When a project/epic has been completed.
/// - `Standby`: When you have desisted at the moment (pause) or maybe something is missing,
/// or even you are editing (only when the edition could be take along time).

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Pending,
    InProgress,
    Done,
    Standby
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identificator {
    pub id: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DoubleIdentificator {
    pub project_id: String,
    pub epic_id: String
}

impl FromStr for Status {
   type Err = ();

   fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Pending" => Ok(Status::Pending),
            "InProgress" => Ok(Status::InProgress),
            "Done" => Ok(Status::Done),
            "Standby" => Ok(Status::Standby),
            _ => Err(())
        }
   }
}

pub fn extract_num(input: &str) -> i32 {
    input.trim().chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
}

pub fn extract_string(input: &str) -> String {
    input.trim().chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<String>()
}

pub fn convert_pattern_to_vec(input: ValueRange) -> Vec<Vec<String>> {
    input
        .values
        .unwrap_or_default()
        .into_iter()
        .map(|element| {
            element[0]
                .as_str()
                .unwrap()
                .split("|")
                .map(|condition| {
                    condition
                        .split("^")
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<Vec<String>>>()
        })
        .flatten()
        .collect::<Vec<Vec<String>>>()
}

pub fn convert_pattern_to_string(input: Vec<Vec<String>>) -> String {
    input
        .iter()
        .map(|row| row.join("^"))
        .collect::<Vec<String>>()
        .join("|")
}
