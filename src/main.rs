use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use juniper::http::graphiql::graphiql_source;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

/// GraphiQL playground UI
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    web::Html::new(graphiql_source("/graphql", None))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(graphql_playground))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
