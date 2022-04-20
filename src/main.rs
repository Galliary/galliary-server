use std::env;

use actix_cors::Cors;
use actix_web::{get, middleware, web, App, HttpServer, Responder};
use anyhow::Result;
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use galliary::{controllers, State};

#[get("/")]
async fn index() -> impl Responder {
    "use /graphql"
}

async fn graphql_handler(req: GraphQLRequest, data: web::Data<State>) -> GraphQLResponse {
    data.schema.execute(req.into_inner()).await.into()
}

#[tokio::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    let db_url = env::var("DATABASE_URL")?;

    let conn = sea_orm::Database::connect(&db_url).await?;
    let schema = Schema::build(
        controllers::Query::default(),
        controllers::Mutation::default(),
        EmptySubscription::default(),
    )
    .data(conn)
    .finish();

    let state = web::Data::new(State::new(schema));

    log::info!("Starting HTTP server...");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
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
