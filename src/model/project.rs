use serde::{Deserialize, Serialize};

use crate::util::util::Status;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertProjectData {
    pub denomination: String,
    pub description: String,
    pub owner: String,
    pub status: Status,
    pub created_at: i64
}

///
/// Project attributes:
///
/// - `id`: _
/// - `denomination`: Name of the project.
/// - `description`: _
/// - `status`: _
/// - `owner`: Fullname of the leader in the project.
/// - `created_at`: Registered date in unix epoch.
/// - `updated_at`: When the owner or somebody with privileges has modify something in the project data.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub denomination: String,
    pub description: String,
    pub owner: String,
    pub status: Status,
    pub created_at: i64,
    pub updated_at: i64,

    pub row: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Denomination {
    pub denomination: String
}