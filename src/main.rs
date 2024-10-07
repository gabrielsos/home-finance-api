mod database;
mod routes;
mod services;
mod dtos;
mod repositories;
mod entities;
mod utils;
mod middlewares;

use dotenv::dotenv;
use actix_web::{App, HttpServer, middleware, web, HttpResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    database::init_db().await;

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(routes::user_routes::init)
            .configure(routes::group_routes::init)
            .default_service(
                web::route().to(||async { HttpResponse::NotFound().body("Rota não encontradaa23") }),
            )
    })
    .bind("0.0.0.0:3059")?
    .run()
    .await
}
