use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Serialize;
extern crate live_reload;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[derive(Serialize)]
struct MyObj {
    name: String,
}

// #[get("/")]
// async fn login() -> Result<impl Responder> {
//     let obj = MyObj {
//         name: "Hello World".to_string(),
//     };
//     Ok(web::Json(obj))
// }

pub async fn login() -> Result<HttpResponse, actix_web::Error> {
    let obj = MyObj {
        name: "Hello World".to_string(),
    };

    Ok(HttpResponse::Ok().json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::resource("/auto").route(web::get().to(login))))
        .bind(("127.0.0.1", 5000))?
        .run()
        .await
}
