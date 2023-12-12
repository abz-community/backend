use crate::graphql::schema::AppSchema;
use actix_web::{App, HttpResponse, HttpServer};
use actix_web::{
    guard,
    web::{self, Data},
};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use graphql::schema::build_schema;

pub mod database;
pub mod graphql;

async fn graphql_handler(schema: Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    let schema_data = build_schema().await;
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema_data.clone()))
            .service(web::resource("/").guard(guard::Post()).to(graphql_handler))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .to(graphql_playground),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
