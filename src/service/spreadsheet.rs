use google_sheets4::api::{AddSheetRequest, BatchUpdateSpreadsheetRequest, DeleteSheetRequest, Request, SheetProperties, Spreadsheet, ValueRange};

use crate::{config::google_sheet_authenticator::get_credentials, util::{cons::SPREADSHEET_ID, util::{extract_num, extract_string}}};

///
/// Create two sheets:
///
/// **1**: A sheet contains a list of projects (Only once will happen this event).
///
/// **2**: The sheet that contains all the epics and user stories.
pub async fn create_sheet(sheet_name: &str) {
    let sheet_credentials = get_credentials()
        .await
        .expect("Could not establish the connection to the spreadsheet");

    let (_, spreadsheet) = sheet_credentials
        .spreadsheets()
        .get(&SPREADSHEET_ID)
        .doit()
        .await
        .unwrap();

    // The main sheet
    let projects_list_exists = spreadsheet
        .sheets
        .clone()
        .unwrap_or_default()
        .iter()
        .any(|sheet| {
            sheet
                .properties
                .as_ref()
                .map(|properties| properties.title.as_deref() == Some(sheet_name))
                .unwrap_or(false)
        });

    //let project_exists = spreadsheet.sheets.unwrap_or_default().iter().any(|sheet| {
    //    sheet
    //        .properties
    //        .as_ref()
    //        .map(|properties| properties.title.as_deref() == Some(project_name))
    //        .unwrap_or(false)
    //});

    if !projects_list_exists {
        // Create the first sheet if does not exists.
        let new_sheet = AddSheetRequest {
            properties: Some(SheetProperties {
                title: Some(sheet_name.to_string()),
                sheet_id: None,
                index: None,
                ..Default::default()
            }),
            ..Default::default()
        };

        // Update the sheets
        let batch_update_request = BatchUpdateSpreadsheetRequest {
            requests: Some(vec![Request {
                add_sheet: Some(new_sheet),
                ..Default::default()
            }]),
            ..Default::default()
        };

        let _ = sheet_credentials
            .spreadsheets()
            .batch_update(batch_update_request, &SPREADSHEET_ID)
            .doit()
            .await;
    }

    //if !project_exists {
    //    // Create the second sheet if does not exists.
    //    let new_sheet = AddSheetRequest {
    //        properties: Some(SheetProperties {
    //            title: Some(project_name.to_string()),
    //            sheet_id: None,
    //            index: None,
    //            ..Default::default()
    //        }),
    //        ..Default::default()
    //    };

    //    // Update the sheets
    //    let batch_update_request = BatchUpdateSpreadsheetRequest {
    //        requests: Some(vec![Request { add_sheet: Some(new_sheet),
    //            ..Default::default()
    //        }]),
    //        ..Default::default()
    //    };

    //    let _ = sheet_credentials
    //        .spreadsheets()
    //        .batch_update(batch_update_request, &SPREADSHEET_ID)
    //        .doit()
    //        .await;
    //}
}

///
/// It returns the identificator of a sheet, e.g: 1307314730.
/// Helping to identify in which sheet you will insert the data into.
pub async fn get_sheet_id(sheet_name: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let sheet_credentials = get_credentials()
        .await
        .expect("Could not establish the connection to the spreadsheet");

    let (_, spreadsheet_metadata): (_, Spreadsheet) = sheet_credentials
        .spreadsheets()
        .get(&SPREADSHEET_ID)
        .doit()
        .await
        .unwrap();

    // Get the sheets properties
    let sheet_id= spreadsheet_metadata
        .sheets
        .ok_or("No sheets found in spreadsheet metadata").unwrap()
        .into_iter()
        .find(|sheet| {
            sheet.properties
                .as_ref()
                .and_then(|props| props.title.as_ref())
                .map_or(false, |title| title == sheet_name) 
        })
        .ok_or_else(|| format!("Sheet with name '{}' not found in spreadsheet.", sheet_name)).unwrap()
        .properties
        .ok_or("Sheet properties not found for the target sheet").unwrap()
        .sheet_id
        .ok_or("Sheet ID not found for the target sheet").unwrap();

    Ok(sheet_id)

}

///
/// It iterates all cells vertically, 
/// then captures the last id and create a new id (new_id = last_id + 1)
pub async fn create_id(sheet_name: &str, cell: &str, id: Option<(&str, &str)>) -> Result<String, Box<dyn std::error::Error>> {
    let sheet_credentials = get_credentials()
        .await
        .expect("Could not establish the connection to the spreadsheet");

    let mut full_range = format!("{}!{}:{}", sheet_name, cell, cell);

    if id.is_some() {
        full_range = format!("{}!{}:{}", sheet_name, cell, id.unwrap().0)
    }

    match sheet_credentials
        .spreadsheets()
        .values_get(&SPREADSHEET_ID, full_range.as_str())
        .doit()
        .await
    {
        Ok((_, value_range)) => {
            let values = value_range.values.unwrap_or_default();

            // Filter empty rows
            let mut non_empty_rows: Vec<Vec<serde_json::Value>> = values
                .into_iter()
                .filter(|row| {
                    row.iter()
                        .any(|cell| !cell.as_str().unwrap_or_default().trim().is_empty())
                })
                .collect();
            
            if non_empty_rows.is_empty() {
                return Ok("null".to_string())
            }

            if id.is_some() {
                non_empty_rows = non_empty_rows
                                    .into_iter()
                                    .filter(|row| row[1].as_str().unwrap_or("") == id.unwrap().1)
                                    .collect();
               
                if non_empty_rows.is_empty() { return Ok("null".to_string())  }
            }

            let last_id = non_empty_rows[non_empty_rows.len() - 1][0].to_string().replace("\"", "");
            
            let new_id = format!(
                "{}{}",
                extract_string(last_id.as_str()),
                extract_num(last_id.as_str()) + 1
            );

            Ok(new_id)
        }
        Err(err) => Err(Box::new(err)),
    }
}

pub async fn read_data(sheet_name: &str, range: (String, String)) -> Vec<Vec<serde_json::Value>> {
    let sheet_credentials = get_credentials()
            .await
            .expect("Could not establish the connection to the spreadsheet");
    
    let full_range = format!("{}!{}:{}", sheet_name, range.0, range.1);

    let value_range: ValueRange = sheet_credentials
        .spreadsheets()
        .values_get(&SPREADSHEET_ID, full_range.as_str())
        .doit()
        .await
        .expect("Could not get the specific data")
        .1;

    value_range.values.unwrap_or_default()
}

pub async fn remove_sheet(sheet_name: &str) {
    let sheet_credentials = get_credentials()
            .await
            .expect("Could not establish the connection to the spreadsheet");
   
    let sheet_id = get_sheet_id(sheet_name).await.unwrap();

    let delete_sheet_request = DeleteSheetRequest {
        sheet_id: Some(sheet_id)
    };

    let request = Request {
        delete_sheet: Some(delete_sheet_request),
        ..Default::default()
    };

    let batch_request = BatchUpdateSpreadsheetRequest {
        requests: Some(vec![request]),
        ..Default::default()
    };

    let _ = sheet_credentials
        .spreadsheets()
        .batch_update(batch_request, &SPREADSHEET_ID)
        .doit()
        .await;
}
