mod api;
mod models;
mod routes;
mod utils;

use core::panic;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use routes::auth::{signin, signup};
use utils::database::db_init;

extern crate live_reload;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = str::parse::<u16>(&dotenv::var("PORT").unwrap()).unwrap();

    let (_, err) = db_init().await;
    if !err.is_empty() {
        panic!("{}", err);
    }

    HttpServer::new(|| App::new().service(web::scope("/auth").service(signin)))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
