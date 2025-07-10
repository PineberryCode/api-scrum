use actix_web::{web, HttpResponse};
use async_trait::async_trait;
use serde::de::DeserializeOwned;

///
/// It encompasess common main functions in a process.
#[async_trait]
pub trait CRUD {
    type CreatePayload: DeserializeOwned + Send + 'static;
    type ReadPayload: DeserializeOwned + Send + 'static;
    type UpdatePayload: DeserializeOwned + Send + 'static;
    type DeletePayload: DeserializeOwned + Send + 'static;

    async fn create(data: web::Json<Self::CreatePayload>) -> HttpResponse;
    async fn read(param: Option<web::Query<Self::ReadPayload>>) -> HttpResponse;
    async fn update(data: web::Json<Self::UpdatePayload>) -> HttpResponse;
    async fn delete(data: web::Json<Self::DeletePayload>) -> HttpResponse;
}