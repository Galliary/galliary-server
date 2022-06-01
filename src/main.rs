#![warn(clippy::all)]

use std::{env, error::Error};

use actix_cors::Cors;
use actix_web::{get, middleware, web, App, HttpServer};
use prisma::PrismaClient;

// use galliary::{controllers, State};
mod prisma;

#[get("/")]
async fn index() -> &'static str {
    "use /graphql"
}

// async fn graphql_handler(req: GraphQLRequest, data: web::Data<State>) -> GraphQLResponse {
//     data.schema.execute(req.into_inner()).await.into()
// }

async fn get_users(client: web::Data<PrismaClient>) -> String {
    let users = client
        .user()
        .find_many(vec![])
        .exec()
        .await
        .expect("failed to fetch data");

    serde_json::to_string(&users).unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    let db_url = env::var("DATABASE_URL")?;
    let host_url = env::var("HOST").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    let client = prisma::new_client_with_url(&db_url).await.unwrap();

    // let conn = sea_orm::Database::connect(&db_url).await?;
    // let schema = Schema::build(
    //     controllers::Query::default(),
    //     controllers::Mutation::default(),
    //     EmptySubscription::default(),
    // )
    // .data(conn)
    // .finish();

    let client = web::Data::new(client);

    log::info!("Starting HTTP server on {host_url}");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::clone(&client))
            .service(
                web::resource("/").route(web::get().to(get_users)), // web::resource("/graphql"), // .route(web::get().to(graphql_handler))
                                                                    // .route(web::post().to(graphql_handler)),
            )
            .service(index)
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .bind(host_url)?
    .run()
    .await?;

    Ok(())
}
