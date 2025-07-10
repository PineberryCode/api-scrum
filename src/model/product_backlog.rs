use std::str::FromStr;

use serde::{Deserialize, Serialize};

///
/// It identifies which type of item is
///
#[derive(Debug, Serialize, Deserialize)]
pub enum ItemType {
    Story,
    Bug,
    Task
}

/// 
/// All items assigned to the product backlog such as stories, bugs and tasks.
/// 
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductBacklogItem {
    pub id: String,
    pub project_id: String,
    pub epic_id: String,
    pub title: String, // Name of the user story or task
    pub priority: String,
    pub description: Option<String>,
    pub points: Option<i64>,
    pub kind: ItemType,
    pub assigned_at: i64,
    pub created_at: i64,
    pub updated_at: i64,

    pub row: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertProductBacklogItem {
    pub project_id: String,
    pub epic_id: String,
    pub title: String,
    pub priority: String,
    pub description: Option<String>,
    pub points: Option<i64>,
    pub kind: ItemType,
    pub assigned_at: i64,
    pub created_at: i64
}

impl FromStr for ItemType {
   type Err = ();

   fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Story" => Ok(ItemType::Story),
            "Bug" => Ok(ItemType::Bug),
            "Task" => Ok(ItemType::Task),
            _ => Err(())
        }
   }
}