use std::str::FromStr;

use actix_web::{HttpResponse, web};
use async_trait::async_trait;
use google_sheets4::api::{ClearValuesRequest, ValueRange};

use crate::{
    config::google_sheet_authenticator::get_credentials,
    interface::crudy::CRUD,
    model::{
        self,
        product_backlog::{InsertProductBacklogItem, ItemType, ProductBacklogItem},
        spreadsheet::{DataRange, Patch},
    },
    service::spreadsheet::create_id,
    util::{
        cons::{PROJECTS_SHEET_NAME, SPREADSHEET_ID},
        util::{extract_num, Identificator},
    },
};

pub struct ProductBacklogService;

#[async_trait]
impl CRUD for ProductBacklogService {
    type CreatePayload = InsertProductBacklogItem;
    type ReadPayload = Identificator;
    type UpdatePayload = Patch;
    type DeletePayload = DataRange;

    async fn create(data: web::Json<Self::CreatePayload>) -> HttpResponse {
        let sheet_credentials = get_credentials()
            .await
            .expect("Could not establish the connection to the spreadsheet");

        let InsertProductBacklogItem {
            project_id,
            epic_id,
            title,
            priority,
            description,
            points,
            kind,
            assigned_at,
            created_at,
        } = data.into_inner();

        let updated_at = created_at.clone();

        let mut item_id = create_id(
            &PROJECTS_SHEET_NAME,
            "AF",
            Some(("AG", project_id.clone().as_str())),
        )
        .await
        .unwrap();

        if item_id == "null" {
            item_id = "PB1".to_string()
        }
        let index = extract_num(item_id.clone().as_str());

        let full_range = format!(
            "{}!AF{}:AP{}",
            &PROJECTS_SHEET_NAME.to_string(),
            index,
            index
        );

        let item_data_vector = vec![
            item_id,
            project_id,
            epic_id,
            title,
            priority,
            description.unwrap_or("".to_string()).to_string(),
            points.unwrap_or(0).to_string(),
            format!("{kind:?}"),
            assigned_at.to_string(),
            created_at.to_string(),
            updated_at.to_string(),
        ];

        let request = ValueRange {
            major_dimension: Some("ROWS".to_string()),
            range: Some(full_range.clone()),
            values: Some(vec![
                item_data_vector
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

        HttpResponse::Ok().json(model::response::Response::<ProductBacklogItem> {
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

        let full_range = format!("{}!AF:AP", &PROJECTS_SHEET_NAME.to_string());

        match sheet_credentials
            .spreadsheets()
            .values_get(&SPREADSHEET_ID, full_range.as_ref())
            .doit()
            .await
        {
            Ok((_, value_range)) => {
                let values = value_range.values.unwrap_or_default();

                let rows: Vec<ProductBacklogItem> = values
                    .into_iter()
                    .enumerate()
                    .filter(|(_, row)| row[1].to_string().replace("\"", "") == id.replace("\"", ""))
                    .filter_map(|(index, row)| {
                        Some(ProductBacklogItem {
                            id: row[0].as_str()?.to_string(),
                            project_id: row[1].as_str()?.to_string(),
                            epic_id: row[2].as_str()?.to_string(),
                            title: row[3].as_str()?.to_string(),
                            priority: row[4].as_str()?.to_string(),
                            description: Some(row[5].as_str()?.to_string()),
                            points: Some(row[6].as_str()?.parse::<i64>().ok()?),
                            kind: ItemType::from_str(row[7].as_str()?).ok().unwrap(),
                            assigned_at: row[8].as_str()?.parse::<i64>().ok()?,
                            created_at: row[9].as_str()?.parse::<i64>().ok()?,
                            updated_at: row[10].as_str()?.parse::<i64>().ok()?,

                            row: format!("AF{}:AP{}", index, index)
                        })
                    })
                    .collect();

                return HttpResponse::Ok().json(model::response::Response::<ProductBacklogItem> {
                    message: "Data was obtained successfully",
                    content: Some(rows),
                    error_bug: None,
                });
            }
            Err(err) => {
                return HttpResponse::InternalServerError().json(model::response::Response::<ProductBacklogItem> {
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

        HttpResponse::Ok().json(model::response::Response::<ProductBacklogItem> {
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

        HttpResponse::Ok().json(model::response::Response::<ProductBacklogItem> {
            message: "It was removed successfully",
            content: None,
            error_bug: None
        })
    }
}