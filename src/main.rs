mod database;
mod dtos;
mod entities;
mod middlewares;
mod repositories;
mod routes;
mod services;
mod utils;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  database::init_db().await;

  HttpServer::new(|| {
    App::new()
      .wrap(middleware::Logger::default())
      .configure(routes::user_routes::init)
      .configure(routes::group_routes::init)
      .default_service(web::route().to(|| async {
        HttpResponse::NotFound().body("Rota não encontradaa23")
      }))
  })
  .bind("0.0.0.0:3059")?
  .run()
  .await
}
