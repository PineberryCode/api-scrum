mod config;
mod model;
mod service;
mod util;
mod interface;

extern crate dotenv;

use actix_cors::Cors;
use actix_web::{App, HttpServer, http::header, web};
use dotenv::dotenv;

use crate::{interface::crudy::CRUD, service::{epic::EpicService, product_backlog::ProductBacklogService, sprint::SprintService}}; 

use rustls::crypto::ring::default_provider;
use service::{project::ProjectService, user_story::UserStoryService};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Supplier Cryptographic
    default_provider()
        .install_default()
        .expect("Failed to set default crypto provider");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:1420")
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
            .allowed_headers(vec![header::ACCEPT, header::CONTENT_TYPE]) // `header` of the actix-web-cors
            .max_age(3600);

        App::new()
            .wrap(cors)
            .route("/projects", web::get().to(<ProjectService as CRUD>::read))
            .route("/project", web::post().to(<ProjectService as CRUD>::create))
            .route("/project", web::patch().to(<ProjectService as CRUD>::update))
            .route("/project", web::delete().to(<ProjectService as CRUD>::delete))
            .route("/epics", web::get().to(<EpicService as CRUD>::read))
            .route("/epic", web::post().to(<EpicService as CRUD>::create))
            .route("/epic", web::patch().to(<EpicService as CRUD>::update))
            .route("/epic", web::delete().to(<EpicService as CRUD>::delete))
            .route("/uss", web::get().to(<UserStoryService as CRUD>::read))
            .route("/us", web::post().to(<UserStoryService as CRUD>::create))
            .route("/us", web::patch().to(<UserStoryService as CRUD>::update))
            .route("/us", web::delete().to(<UserStoryService as CRUD>::delete))
            .route("/pbs", web::get().to(<ProductBacklogService as CRUD>::read))
            .route("/pb", web::post().to(<ProductBacklogService as CRUD>::create))
            .route("/pb", web::patch().to(<ProductBacklogService as CRUD>::update))
            .route("/pb", web::delete().to(<ProductBacklogService as CRUD>::delete))
            .route("/sprints", web::get().to(<SprintService as CRUD>::read))
            .route("/sprint", web::post().to(<SprintService as CRUD>::create))
            .route("/sprint", web::post().to(<SprintService as CRUD>::update))
            .route("/sprint", web::post().to(<SprintService as CRUD>::delete))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
