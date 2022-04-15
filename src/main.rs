use std::{env, sync::Arc};

use actix_cors::Cors;
use actix_web::{
    get, middleware, route,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use anyhow::Result;
use sea_orm::{entity::*, query::*, DatabaseConnection};

mod entity;

use entity::prelude::*;

#[derive(Clone)]
pub struct State {
    conn: DatabaseConnection,
}

#[get("/")]
async fn index(req: HttpRequest, data: web::Data<State>) -> impl Responder {
    if let Ok(users) = User::find().all(&data.conn).await {
        HttpResponse::Ok().body(users.len().to_string())
    } else {
        HttpResponse::InternalServerError().body("Error Looking up".to_string())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    let db_url = env::var("DATABASE_URL")?;

    let conn = sea_orm::Database::connect(&db_url).await?;

    let state = State { conn };

    log::info!("Starting HTTP server...");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(index)
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
