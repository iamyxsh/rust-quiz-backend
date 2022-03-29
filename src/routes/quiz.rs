use actix_web::{post, web, HttpRequest, HttpResponse};

use crate::{
    routes::{error_bodies::ErrorResponse, request_bodies},
    utils::jwt::get_id,
};

#[post("/")]
async fn create_quiz(
    req: HttpRequest,
    body: web::Json<request_bodies::CreateQuizBody>,
) -> HttpResponse {
    let token = req.headers().get("Authorization");
    match token {
        Some(token) => {
            let user_id = get_id(token.to_str().unwrap().to_string());
            match user_id {
                Ok(res) => todo!(),
                Err(_) => todo!(),
            }
        }
        None => HttpResponse::Unauthorized().json(ErrorResponse {
            status: false,
            message: "Unauthorized request.".to_string(),
        }),
    }
}
