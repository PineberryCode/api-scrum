use std::str::FromStr;

use actix_web::{HttpResponse, web};
use async_trait::async_trait;
use google_sheets4::api::{ClearValuesRequest, ValueRange};

use crate::{
    config::google_sheet_authenticator::get_credentials, interface::crudy::CRUD, model::{
        self, spreadsheet::{SpecificDataRange, SpecificPatch}, user_story::{InsertUserStoryBox, Scenario, ScenarioType, UserStory, UserStoryBox}
    }, util::{
        cons::{PROJECTS_SHEET_NAME, SPREADSHEET_ID},
        util::{convert_pattern_to_string, convert_pattern_to_vec, extract_num, DoubleIdentificator},
    }
};

use super::spreadsheet::create_id;

pub struct UserStoryService;

#[async_trait]
impl CRUD for UserStoryService {
    type CreatePayload = InsertUserStoryBox;
    type ReadPayload = DoubleIdentificator;
    type UpdatePayload = SpecificPatch;
    type DeletePayload = SpecificDataRange;

    async fn create(data: web::Json<Self::CreatePayload>) -> HttpResponse {
        let sheet_credentials = get_credentials()
            .await
            .expect("Could not establish the connection to the spreadsheet");

        let InsertUserStoryBox {
            project_id,
            epic_id,
            title,
            priority,
            story_points,
            user_story:
                UserStory {
                    role,
                    functionality,
                    benefit,
                },
            acceptance_criteria: (scenarios, details),
            created_at,
        } = data.into_inner();
        
        let updated_at = created_at.clone();

        let mut user_story_id = create_id(
                &PROJECTS_SHEET_NAME,
                "R",
                Some(("S", project_id.clone().as_str()))
            )
            .await
            .unwrap();

        if user_story_id == "null" { user_story_id = "US1".to_string() }
        let index = extract_num(user_story_id.clone().as_str());


        let fmt_scenarios = scenarios
            .into_iter()
            .enumerate()
            .map(|(index, scenario)| {
                return format!(
                    "{}:{:?}^{}^{}^{}",
                    index, scenario.kind, scenario.given, scenario.when, scenario.then
                );
            })
            .collect::<Vec<String>>()
            .join("|");

        let fmt_details = details
            .into_iter()
            .enumerate()
            .map(|(index, detail)| return format!("{}:{}", index, detail))
            .collect::<Vec<String>>()
            .join("|");

        let full_range = format!(
            "{}!R{}:AD{}",
            &PROJECTS_SHEET_NAME.to_string(),
            index,
            index
        );

        let user_story_vector = vec![
            user_story_id,
            project_id,
            epic_id,
            title,
            priority,
            story_points.to_string(),
            role,
            functionality,
            benefit,
            fmt_scenarios,
            fmt_details,
            created_at.to_string(),
            updated_at.to_string()
        ];

        let request = ValueRange {
            major_dimension: Some("ROWS".to_string()),
            range: Some(full_range.clone()),
            values: Some(vec![
                user_story_vector
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

        let DoubleIdentificator { project_id, epic_id } = param.unwrap().into_inner();

        let full_range = format!("{}!R:AD", &PROJECTS_SHEET_NAME.to_string());

        match sheet_credentials
            .spreadsheets()
            .values_get(&SPREADSHEET_ID, full_range.as_ref())
            .doit()
            .await
        {
            Ok((_, value_range)) => {
                let values = value_range.values.unwrap_or_default();

                let rows: Vec<UserStoryBox> = values
                    .into_iter()
                    .enumerate()
                    .filter(|(_, row)| row[1].to_string().replace("\"", "") == project_id.replace("\"", "") && row[2].to_string().replace("\"", "") == epic_id.replace("\"",""))
                    .filter_map(|(index, row)| {
                        let scenarios_value_range = ValueRange {
                           range: None,
                           major_dimension: None,
                           values: Some(vec![vec![row.get(9).cloned().unwrap_or_default()]]),
                           ..Default::default()
                        };

                        let details_value_range = ValueRange {
                            range: None,
                            major_dimension: None,
                            values: Some(vec![vec![row.get(10).cloned().unwrap_or_default()]]),
                            ..Default::default()
                        };

                        let scenarios: Vec<Scenario> = convert_pattern_to_vec(scenarios_value_range)
                            .into_iter()
                            .map(|val| {
                                let scenario_string = val.get(0)
                                    .cloned()
                                    .unwrap_or_default();

                                let scenario_kind = scenario_string
                                    .as_str()
                                    .split(":")
                                    .collect::<Vec<&str>>();

                                return Scenario {
                                    kind: ScenarioType::from_str(scenario_kind[1]).ok().unwrap(),
                                    given: val.get(1).cloned().unwrap_or_default(),
                                    when: val.get(2).cloned().unwrap_or_default(),
                                    then: val.get(3).cloned().unwrap_or_default()
                                }
                            })
                            .collect();

                        let details: Vec<String> = convert_pattern_to_vec(details_value_range)
                            .get(0)
                            .cloned()
                            .unwrap_or_default();

                        Some(UserStoryBox {
                            id: row[0].as_str()?.to_string(),
                            project_id: row[1].as_str()?.to_string(),
                            epic_id: row[2].as_str()?.to_string(),
                            title: row[3].as_str()?.to_string(),
                            priority: row[4].as_str()?.to_string(),
                            story_points: row[5].as_str()?.parse().ok()?,
                            user_story: UserStory {
                                role: row[6].as_str()?.to_string(),
                                functionality: row[7].as_str()?.to_string(),
                                benefit: row[8].as_str()?.to_string()
                            },
                            acceptance_criteria: (scenarios, details),
                            created_at: row[11].as_str()?.parse().ok()?,
                            updated_at: row[12].as_str()?.parse().ok()?,

                            row: format!("R{}:AD{}", index, index)
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
                    "response": format!("Error reading the sheet: {}", err)
                }));
            }
        }
    }

    async fn update(data: web::Json<Self::UpdatePayload>) -> HttpResponse {
        let sheet_credentials = get_credentials()
            .await
            .expect("Could not establish the connection to the spreadsheet");

        let SpecificPatch {
            content,
            cell,
            index,
        } = data.into_inner();

        let full_range = format!("{}!{}:{}", &PROJECTS_SHEET_NAME.to_string(), cell, cell);

        if (cell.contains("AA") || cell.contains("AB")) && index.is_some() {
            let (_, value_range) = sheet_credentials
                .spreadsheets()
                .values_get(&SPREADSHEET_ID, full_range.clone().as_str())
                .doit()
                .await
                .expect("Could not get data");

            let mut str_to_vector: Vec<Vec<String>> = convert_pattern_to_vec(value_range);

            if index.unwrap() > str_to_vector.len() {
                let fmt_content = format!("{}:{}", str_to_vector.len(), content).to_string();
                str_to_vector.push([fmt_content].to_vec());
            } else if index.unwrap() <= str_to_vector.len() {
                let fmt_content = format!("{}:{}", index.unwrap(), content).to_string();
                str_to_vector[index.unwrap()] = [fmt_content].to_vec();
            }

            let vector_to_str = convert_pattern_to_string(str_to_vector);

            let input = vec![vector_to_str];

            let request = ValueRange {
                major_dimension: Some("ROWS".to_string()),
                range: Some(full_range.clone()),
                values: Some(vec![
                    input
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
                .expect("Could not removed the data in the sheet");
        }

        if index.is_none() {
            let us_data = vec![content.clone()];

            let request = ValueRange {
                major_dimension: Some("ROWS".to_string()),
                range: Some(full_range.clone()),
                values: Some(vec![
                    us_data
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
        }

        HttpResponse::Ok().json(serde_json::json!({
            "response": "Data was updated in the sheet successfully!"
        }))
    }

    async fn delete(data: web::Json<Self::DeletePayload>) -> HttpResponse {
        let sheet_credentials = get_credentials()
            .await
            .expect("Could not establish the connection to the spreadsheet");

        let SpecificDataRange {
            range: (start, end),
            index,
        } = data.into_inner();

        let range_str = format!("{}!{}:{}", &PROJECTS_SHEET_NAME.to_string(), start, end);

        if (start.contains("AA") && end.contains("AA"))
            || (start.contains("AB") && end.contains("AB")) && index.is_some()
        {
            let (_, value_range) = sheet_credentials
                .spreadsheets()
                .values_get(&SPREADSHEET_ID, range_str.clone().as_str())
                .doit()
                .await
                .expect("Could not get data");

            let str_to_vector: Vec<Vec<String>> = value_range
                .values
                .unwrap_or_default()
                .into_iter()
                .map(|element| {
                    element[0]
                        .as_str()
                        .unwrap()
                        .split("|")
                        .map(|condition| {
                            condition
                                .split("^")
                                .map(|s| s.to_string())
                                .collect::<Vec<String>>()
                        })
                        .collect::<Vec<Vec<String>>>()
                })
                .flatten()
                .collect();

            let filter_values = str_to_vector
                .iter()
                .filter(|row| {
                    row[0].as_str().split(":").collect::<Vec<&str>>()[0]
                        != index.unwrap().to_string().as_str()
                })
                .map(|element| element.clone())
                .collect::<Vec<Vec<String>>>();

            let vector_to_str = filter_values
                .iter()
                .map(|row| row.join("^"))
                .collect::<Vec<String>>()
                .join("|");

            let input = vec![vector_to_str];

            let request = ValueRange {
                major_dimension: Some("ROWS".to_string()),
                range: Some(range_str.clone()),
                values: Some(vec![
                    input
                        .iter()
                        .map(|element| Into::into(element.as_str()))
                        .collect(),
                ]),
            };

            sheet_credentials
                .spreadsheets()
                .values_update(request, &SPREADSHEET_ID, range_str.as_str())
                .value_input_option("USER_ENTERED")
                .doit()
                .await
                .expect("Could not removed the data in the sheet");
        }

        if index.is_none() {
            sheet_credentials
                .spreadsheets()
                .values_clear(ClearValuesRequest::default(), &SPREADSHEET_ID, &range_str)
                .doit()
                .await
                .expect("Could not removed the data in the sheet");
        }

        HttpResponse::Ok().json(serde_json::json!({
            "response": "It was removed successfully!"
        }))
    }
}