use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

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
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
