use std::env;

use actix_cors::Cors;
use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use async_graphql::connection::EmptyFields;
use async_graphql::{extensions::Extension, EmptyMutation};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use sea_orm::{entity::*, query::*, DatabaseConnection};

mod db;
mod entity;

use db::State;

use entity::prelude::*;

type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

#[derive(async_graphql::MergedObject, Default)]
pub struct Query(String);

#[get("/")]
async fn index(_req: HttpRequest, data: web::Data<State>) -> impl Responder {
    ""
}

async fn graphql_handler(_req: HttpRequest) -> impl Responder {
    "hi"
}

#[tokio::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    let db_url = env::var("DATABASE_URL")?;

    let conn = sea_orm::Database::connect(&db_url).await?;
    let schema = Schema::build(
        Query::default(),
        EmptyMutation::default(),
        EmptySubscription::default(),
    )
    .data(conn)
    .finish();

    let state = State::new(schema);

    log::info!("Starting HTTP server...");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(
                web::resource("/graphql")
                    .route(web::get().to(graphql_handler))
                    .route(web::post().to(graphql_handler)),
            )
            .service(index)
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
