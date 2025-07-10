use serde::{Deserialize, Serialize};

use crate::util::util::Status;

///
/// Sprint attributes:
///
/// - `id`: _
/// - `project_id`: _
/// - `epic_id`: _
/// - `denomination`: The name or title of the sprint.
/// - `description`: _
/// - `goal`: The target of the sprint.
/// - `owner`: Who is the leader.
/// - `estimated_points`: The total of points (story points or points in general like a *task*).
/// - `status`: Whether the sprint is **Pending**, **InProgress**, **Done**, **Standby**.
/// - `completed_at`: When the sprint has culminated.
/// - `start_date`: The initial date in unix epoch format.
/// - `end_date`: The end date in unix epoch format
/// - `created_at`: Registered date in unix epoch by somebody.
/// - `updated_at`: Updated date in unix epoch by somebody
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sprint {
    pub id: String,
    pub project_id: String,
    pub epic_id: String,
    pub denomination: String,
    pub description: Option<String>,
    pub goal: String,
    pub owner: String,
    pub estimated_points: i64,
    pub status: Status,
    pub completed_at: i64,
    pub start_date: i64,
    pub end_date: i64,
    pub created_at: i64,
    pub updated_at: i64,

    pub row: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertSprint {
    pub project_id: String,
    pub epic_id: String,
    pub denomination: String,
    pub description: Option<String>,
    pub goal: String,
    pub owner: String,
    pub estimated_points: i64,
    pub status: Status,
    pub completed_at: i64,
    pub start_date: i64,
    pub end_date: i64,
    pub created_at: i64
}