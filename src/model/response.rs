use serde::{Deserialize, Serialize};

///
/// It is a custom response for fuctions that returns a HttpResponse
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    pub message: &'static str,
    pub content: Option<Vec<T>>,
    pub error_bug: Option<Vec<String>>
}