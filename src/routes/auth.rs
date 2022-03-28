use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    models::user::User,
    routes::{error_bodies::ErrorResponse, request_bodies},
};

#[derive(Debug, Deserialize, Serialize)]
struct CreateUserRes {
    status: bool,
    payload: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct GetUser {
    user: User,
}

#[post("/signup")]
async fn signup(body: web::Json<request_bodies::SignupBody>) -> HttpResponse {
    let err = body.validate().err();
    let mut err_string = "".to_string();
    if err.is_some() {
        for e in err.into_iter() {
            err_string = e.to_string();
            break;
        }
        HttpResponse::BadGateway().json(ErrorResponse {
            status: false,
            message: err_string,
        })
    } else {
        let user = User {
            id: None,
            email: body.email.to_string(),
            name: Some(body.name.to_string()),
            password: body.password.to_string(),
        };
        let result = user.insert_user().await;
        match result {
            Ok(_) => HttpResponse::Created().json(CreateUserRes {
                status: true,
                payload: "User Created".to_string(),
            }),
            Err(err) => HttpResponse::BadGateway().json(ErrorResponse {
                status: false,
                message: err.to_string(),
            }),
        }
    }
}

#[post("/signin")]
async fn signin(body: web::Json<request_bodies::SigninBody>) -> HttpResponse {
    let err = body.validate().err();
    let mut err_string = "".to_string();
    if err.is_some() {
        for e in err.into_iter() {
            err_string = e.to_string();
            break;
        }
        HttpResponse::BadGateway().json(ErrorResponse {
            status: false,
            message: err_string,
        })
    } else {
        let mut user = User {
            id: None,
            name: None,
            email: body.email.to_string(),
            password: body.password.to_string(),
        };

        let tx = user
            .get_user("email".to_string(), body.email.to_string())
            .await;
        match tx {
            Ok(_) => {
                if user.id.is_some() {
                    HttpResponse::Ok().json(GetUser { user: user })
                } else {
                    HttpResponse::NotFound().body("false")
                }
            }
            Err(e) => HttpResponse::BadGateway().json(ErrorResponse {
                status: false,
                message: e.to_string(),
            }),
        }
    }
}
