use std::str::FromStr;

use actix_web::{HttpResponse, web};
use async_trait::async_trait;
use google_sheets4::api::{ClearValuesRequest, ValueRange};

use crate::{
    config::google_sheet_authenticator::get_credentials,
    interface::crudy::CRUD,
    model::{
        self,
        spreadsheet::{DataRange, Patch},
        sprint::{InsertSprint, Sprint},
    },
    service::spreadsheet::create_id,
    util::{
        cons::{PROJECTS_SHEET_NAME, SPREADSHEET_ID},
        util::{Identificator, Status, extract_num},
    },
};

pub struct SprintService;

#[async_trait]
impl CRUD for SprintService {
    type CreatePayload = InsertSprint;
    type ReadPayload = Identificator;
    type UpdatePayload = Patch;
    type DeletePayload = DataRange;

    async fn create(data: web::Json<Self::CreatePayload>) -> HttpResponse {
        let sheet_credentials = get_credentials()
            .await
            .expect("Could not establish the connection to the spreadsheet");

        let InsertSprint {
            project_id,
            epic_id,
            denomination,
            description,
            goal,
            owner,
            estimated_points,
            status,
            completed_at,
            start_date,
            end_date,
            created_at,
        } = data.into_inner();

        let updated_at = created_at.clone();

        let mut sprint_id = create_id(
            &PROJECTS_SHEET_NAME,
            "AR",
            Some(("AS", project_id.clone().as_str())),
        )
        .await
        .unwrap();

        if sprint_id == "null" {
            sprint_id = "S".to_string()
        }
        let index = extract_num(sprint_id.clone().as_str());

        let full_range = format!(
            "{}!AR{}:BE{}",
            &PROJECTS_SHEET_NAME.to_string(),
            index,
            index
        );

        let sprint_data_vector = vec![
            sprint_id,
            project_id,
            epic_id,
            denomination,
            description.unwrap_or("".to_string()),
            goal,
            owner,
            estimated_points.to_string(),
            format!("{status:?}"),
            completed_at.to_string(),
            start_date.to_string(),
            end_date.to_string(),
            created_at.to_string(),
            updated_at.to_string(),
        ];

        let request = ValueRange {
            major_dimension: Some("ROWS".to_string()),
            range: Some(full_range.clone()),
            values: Some(vec![
                sprint_data_vector
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

        HttpResponse::Ok().json(model::response::Response::<InsertSprint> {
            message: "Data was inserted in the sheet successfully!",
            content: None,
            error_bug: None,
        })
    }

    async fn read(param: Option<web::Query<Self::ReadPayload>>) -> HttpResponse {
        let sheet_credentials = get_credentials()
            .await
            .expect("Could not establish the connection to the spreadsheet");

        let Identificator { id } = param.unwrap().into_inner();

        let full_range = format!("{}!AR:BE", &PROJECTS_SHEET_NAME.to_string());

        match sheet_credentials
            .spreadsheets()
            .values_get(&SPREADSHEET_ID, full_range.as_ref())
            .doit()
            .await
        {
            Ok((_, value_range)) => {
                let values = value_range.values.unwrap_or_default();

                let rows: Vec<Sprint> = values
                    .into_iter()
                    .filter(|row| row[1].to_string().replace("\"", "") == id.replace("\"", ""))
                    .enumerate()
                    .filter_map(|(index, row)| {
                        Some(Sprint {
                            id: row[0].as_str()?.to_string(),
                            project_id: row[1].as_str()?.to_string(),
                            epic_id: row[2].as_str()?.to_string(),
                            denomination: row[3].as_str()?.to_string(),
                            description: Some(row[4].as_str()?.to_string()),
                            goal: row[5].as_str()?.to_string(),
                            owner: row[6].as_str()?.to_string(),
                            estimated_points: row[7].as_str()?.parse::<i64>().ok()?,
                            status: Status::from_str(row[8].as_str()?).ok().unwrap(),
                            completed_at: row[9].as_str()?.parse::<i64>().ok()?,
                            start_date: row[10].as_str()?.parse::<i64>().ok()?,
                            end_date: row[11].as_str()?.parse::<i64>().ok()?,
                            created_at: row[12].as_str()?.parse::<i64>().ok()?,
                            updated_at: row[13].as_str()?.parse::<i64>().ok()?,

                            row: format!("AR{}:BE{}", index, index)
                        })
                    })
                    .collect();

                return HttpResponse::Ok().json(model::response::Response::<Sprint> {
                    message: "Data was obtained successfully",
                    content: Some(rows),
                    error_bug: None,
                });
            }
            Err(err) => {
                return HttpResponse::InternalServerError().json(model::response::Response::<Sprint> {
                    message: "Error reading the sheet",
                    content: None,
                    error_bug: Some(vec![err.to_string()]),
                });
            }
        }
    }

    async fn update(data: web::Json<Self::UpdatePayload>) -> HttpResponse {
        let sheet_credentials = get_credentials()
            .await
            .expect("Could not establish the connection to the spreadsheet");

        let Patch { content, cell } = data.into_inner();

        let full_range = format!("{}!{}:{}", &PROJECTS_SHEET_NAME.to_string(), cell, cell);

        let item_partial_data = vec![content];

        let request = ValueRange {
            major_dimension: Some("ROWS".to_string()),
            range: Some(full_range.clone()),
            values: Some(vec![
                item_partial_data
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

        HttpResponse::Ok().json(model::response::Response::<Sprint> {
            message: "Data was updated in the sheet successfully!",
            content: None,
            error_bug: None,
        })
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

        HttpResponse::Ok().json(model::response::Response::<Sprint> {
            message: "It was removed successfully",
            content: None,
            error_bug: None,
        })
    }
}