use std::str::FromStr;

use actix_web::{HttpResponse, web};
use async_trait::async_trait;
use google_sheets4::api::{ClearValuesRequest, ValueRange};

use crate::{
    config::google_sheet_authenticator::get_credentials,
    interface::crudy::CRUD,
    model::{
        self, epic::{Epic, InsertEpicData}, spreadsheet::{DataRange, Patch}
    },
    service::spreadsheet::create_id,
    util::{
        cons::{PROJECTS_SHEET_NAME, SPREADSHEET_ID},
        util::{extract_num, Identificator, Status},
    },
};

pub struct EpicService;

#[async_trait]
impl CRUD for EpicService {
    type CreatePayload = InsertEpicData;
    type ReadPayload = Identificator;
    type UpdatePayload = Patch;
    type DeletePayload = DataRange;

    async fn create(data: web::Json<Self::CreatePayload>) -> HttpResponse {
        let sheet_credentials = get_credentials()
            .await
            .expect("Could not establish the connection to the spreadsheet");

        let InsertEpicData {
            project_id,
            title,
            description,
            status,
            owner,
            created_at,
        } = data.into_inner();

        let mut epic_id = create_id(
            &PROJECTS_SHEET_NAME,
            "I",
            Some(("J", project_id.clone().as_str())),
        )
        .await
        .unwrap();

        if epic_id == "null" { epic_id = "E1".to_string() }
        let index = extract_num(epic_id.clone().as_str());

        let full_range = format!("{}!I{}:P{}", &PROJECTS_SHEET_NAME.to_string(), index, index);

        let updated_at = created_at.clone();

        let epic_data_vector = vec![
            epic_id.clone(),
            project_id,
            title,
            description,
            format!("{status:?}"),
            owner,
            created_at.to_string(),
            updated_at.to_string(),
        ];

        let request = ValueRange {
            major_dimension: Some("ROWS".to_string()),
            range: Some(full_range.clone()),
            values: Some(vec![
                epic_data_vector
                    .iter()
                    .map(|element| Into::into(element.as_str()))
                    .collect(),
            ]),
        };

        sheet_credentials
            .spreadsheets()
            .values_append(request, &SPREADSHEET_ID, full_range.as_str())
            .value_input_option("USER_ENTERED")
            .doit()
            .await
            .expect("Could not insert the data in the sheet");

        HttpResponse::Ok().json(serde_json::json!({
            "response": "Data was inserted in the sheet successfully!"
        }))
    }

    async fn read(param: Option<web::Query<Self::ReadPayload>>) -> HttpResponse {
        let sheet_credentials = get_credentials()
            .await
            .expect("Could not establish the connection to the spreadsheet");

        let Identificator { id } = param.unwrap().into_inner();

        let full_range = format!("{}!I:R", &PROJECTS_SHEET_NAME.to_string());

        match sheet_credentials
            .spreadsheets()
            .values_get(&SPREADSHEET_ID, full_range.as_ref())
            .doit()
            .await
        {
            Ok((_, value_range)) => {
                let values = value_range.values.unwrap_or_default();

                // Filter empty rows
                let rows: Vec<Epic> = values
                    .into_iter()
                    .enumerate()
                    .filter(|(_, row)| row[1].to_string().replace("\"", "") == id.replace("\"", ""))
                    .filter_map(|(index, row)| {
                        Some(Epic {
                            id: row[0].as_str()?.to_string(),
                            project_id: row[1].as_str()?.to_string(),
                            title: row[2].as_str()?.to_string(),
                            description: row[3].as_str()?.to_string(),
                            status: Status::from_str(row[4].as_str()?).ok()?,
                            owner: row[5].as_str()?.to_string(),
                            created_at: row[6].as_str()?.parse().ok()?,
                            updated_at: row[7].as_str()?.parse().ok()?,

                            row: format!("I{}:R{}", index, index)
                        })
                    })
                    .collect();

                return HttpResponse::Ok().json(
                    model::response::Response {
                        message: "Data was obtained successfully",
                        content: Some(rows),
                        error_bug: None
                    }
                );
            }
            Err(err) => {
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": format!("Error reading the sheet: {}", err)
                }));
            }
        }
    }

    async fn update(data: web::Json<Self::UpdatePayload>) -> HttpResponse {
        let sheet_credentials = get_credentials()
            .await
            .expect("Could not establish the connection to the spreadsheet");

        let Patch { content, cell } = data.into_inner();

        let full_range = format!("{}!{}:{}", &PROJECTS_SHEET_NAME.to_string(), cell, cell);

        let epic_data = vec![content];

        let request = ValueRange {
            major_dimension: Some("ROWS".to_string()),
            range: Some(full_range.clone()),
            values: Some(vec![
                epic_data
                    .iter()
                    .map(|element| Into::into(element.as_str()))
                    .collect(),
            ]),
        };

        sheet_credentials
            .spreadsheets()
            .values_update(request, &SPREADSHEET_ID, full_range.as_str())
            .value_input_option("USER_ENTERED")
            .doit()
            .await
            .expect("Could not update the data in the sheet");

        HttpResponse::Ok().json(serde_json::json!({
            "response": "Data was updated in the sheet successfully!"
        }))
    }

    async fn delete(data: web::Json<Self::DeletePayload>) -> HttpResponse {
        let sheet_credentials = get_credentials()
            .await
            .expect("Could not establish the connection to the spreadsheet");

        let DataRange { range } = data.into_inner();

        let range_str = format!(
            "{}!{}:{}",
            &PROJECTS_SHEET_NAME.to_string(),
            range.0,
            range.1
        );

        sheet_credentials
            .spreadsheets()
            .values_clear(ClearValuesRequest::default(), &SPREADSHEET_ID, &range_str)
            .doit()
            .await
            .expect("Could not removed the data in the sheet");

        HttpResponse::Ok().json(serde_json::json!({
            "response": "Epic was removed successfully!"
        }))
    }
}