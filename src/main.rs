mod database;
mod routes;
mod services;
mod dtos;

use dotenv::dotenv;
use std::env;
use actix_web::{App, HttpServer, middleware, web, HttpResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "default_value".to_string());

    match database::MongoDB::init(&database_url, "home-finance").await {
        Ok(_) => println!("Conexão com MongoDB estabelecida com sucesso."),
        Err(e) => {
            eprintln!("Erro ao conectar ao MongoDB: {}", e);
            std::process::exit(1);
        }
    }

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(routes::user_routes::init)
            .default_service(
                web::route().to(||async { HttpResponse::NotFound().body("Rota não encontradaa23") }),
            )
    })
    .bind("0.0.0.0:3058")?
    .run()
    .await
}
