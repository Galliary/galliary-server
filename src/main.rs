use std::{io, sync::Arc};

use actix_cors::Cors;
use actix_web::{
    get, middleware, route,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use actix_web_lab::respond::Html;
use diesel::QueryResult;
use juniper::FieldResult;
use juniper::http::{GraphQLRequest};
use juniper::meta::Field;
use serde_json::Value::Array;

mod lib;
mod models;
mod structs;
mod controllers;

use controllers::album::{get_album, get_albums};
use crate::models::album::Album;

#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let connection = lib::connect();
    let user = data.execute(&st, &(connection)).await;
    HttpResponse::Ok().json(user)
}

impl QueryRoot {
    pub fn get_album(&self) -> FieldResult<Album> {
        get_album()
    }
    pub fn get_albums(&self) -> FieldResult<Vec<Album>> {
        get_albums()
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Create Juniper schema
    let schema = Arc::new(create_schema());

    log::info!("Starting HTTP server...");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(
                Schema::new(
                    QueryRoot,
                    juniper::EmptyMutation::new(),
                    juniper::EmptySubscription::new(),
                )))
            .service(graphql)
            // the graphiql UI requires CORS to be enabled
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
        .workers(2)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}