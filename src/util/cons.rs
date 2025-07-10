use std::env;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref SPREADSHEET_ID: String = env::var("SPREADSHEET_ID").unwrap();
    pub static ref PROJECTS_SHEET_NAME: String = "Projects".to_string();
}
