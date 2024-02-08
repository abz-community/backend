use crate::graphql::schema::AppSchema;
use actix_cors::Cors;
use actix_web::{
    guard,
    web::{self, Data},
};
use actix_web::{App, HttpResponse, HttpServer};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use graphql::schema::build_schema;
use log::info;

pub mod database;
pub mod graphql;
pub mod local_storage;
use actix_web::{get, Responder};

async fn graphql_handler(schema: Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[get("/ping")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "main");
    }
    env_logger::init();
    info!("Initializing schema");
    let schema_data = build_schema().await;
    info!("Db Ok");
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(Data::new(schema_data.clone()))
            .service(hello)
            .service(web::resource("/").guard(guard::Post()).to(graphql_handler))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .to(graphql_playground),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
