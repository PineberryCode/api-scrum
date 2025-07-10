use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// It determines whether a scenario is success or not.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ScenarioType {
    Success,
    Failure
}

/// Scenario attributes:
///
/// - `given`: how things begin.
/// - `when`: action taken.
/// - `then`: outcome of taking action.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scenario {
    pub kind: ScenarioType,
    pub given: String,
    pub when: String,
    pub then: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStoryBox {
    pub id: String,
    pub project_id: String,
    pub epic_id: String,
    pub title: String,
    pub priority: String,
    pub story_points: i32,
    pub user_story: UserStory,
    pub acceptance_criteria: (Vec<Scenario>, DetailsList),
    pub created_at: i64,
    pub updated_at: i64,

    pub row: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertUserStoryBox {
    pub project_id: String,
    pub epic_id: String,
    pub title: String,
    pub priority: String,
    pub story_points: i32,
    pub user_story: UserStory,
    pub acceptance_criteria: (Vec<Scenario>, DetailsList),
    pub created_at: i64,
}

///
/// User Story attributes:
///
/// - `story_points`: Complexity or technical risk.
///
/// - `role`: As a/n [description of user].
///
/// - `functionality`: I want [functionality].
///
/// - `benefit`: So that [benefit].

#[derive(Debug, Serialize, Deserialize)]
pub struct UserStory {
    pub role: String,
    pub functionality: String,
    pub benefit: String
}

///
/// This covers functional requirements, non-functional requirements, etc.
type Stuff = String;
type DetailsList = Vec<Stuff>;

impl FromStr for ScenarioType {
   type Err = ();

   fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Success" => Ok(ScenarioType::Success),
            "Failure" => Ok(ScenarioType::Failure),
            _ => Err(())
        }
   }
}
