use serde::{Deserialize, Serialize};

use crate::util::util::Status;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertEpicData {
    pub project_id: String,
    pub title: String,
    pub description: String,
    pub status: Status,
    pub owner: String,
    pub created_at: i64
}

/// Epic attributes:
///
/// - `id`: Identificator and num of the epic.
/// - `project_id`: _
/// - `title`: Name of the epic.
/// - `description`: _ 
/// - `status`: _
/// - `owner`: Fullname of the leader in the epic.
/// - `created_at`: Registered date in unix epoch by the owner.
/// - `updated_at`: When the owner or somebody with privileges has modify something in the epic details.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Epic {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub description: String,
    pub status: Status,
    pub owner: String,
    pub created_at: i64,
    pub updated_at: i64,

    pub row: String
}
