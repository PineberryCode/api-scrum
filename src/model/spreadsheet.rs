use serde::{Deserialize, Serialize};

/// 
/// It determines the position of an specific cell.
/// 
/// `position`: e.g- A1, B3, H3.
pub type CellPosition = String;

/// Coodinates in the spreadsheet.
type Y = String;
type X = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct DataRange {
    pub range: (Y, X)
}

///
/// SpecificDataRange attributes:
///
/// - `range`: _,
/// - `index`: Indicates which element of a string must be modified.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecificDataRange {
    pub range: (Y, X),
    pub index: Option<i64>
}

///
/// It modifies a specific data of a project, epic, etc.
#[derive(Debug, Serialize, Deserialize)]
pub struct Patch {
    pub content: String,
    pub cell: CellPosition
}

///
/// It modifies a specific data of a project, epic, etc.
///
/// SpecificPatch attributes:
///
/// - `index`: Indicates which element of a string must be modified.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificPatch {
    pub content: String,
    pub cell: CellPosition,
    pub index: Option<usize>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetName {
    pub sheet_name: String
}
