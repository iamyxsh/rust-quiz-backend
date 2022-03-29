use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    models::user::User,
    routes::{error_bodies::ErrorResponse, request_bodies},
    utils::{
        hashing::{gen_hash, verify_hash},
        jwt::gen_token,
    },
};

#[derive(Debug, Deserialize, Serialize)]
struct SignUpRes {
    status: bool,
    payload: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct SignInRes {
    status: bool,
    payload: String,
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
        let password_hash = gen_hash(body.password.to_string()).unwrap();
        let user = User {
            id: None,
            email: body.email.to_string(),
            name: Some(body.name.to_string()),
            password: password_hash,
        };
        let result = user.insert_user().await;
        match result {
            Ok(_) => HttpResponse::Created().json(SignUpRes {
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
                    if verify_hash(user.password, body.password.to_string()) {
                        let token = gen_token(format!("{}", user.id.unwrap()));
                        match token {
                            Ok(token) => HttpResponse::Ok().json(SignInRes {
                                status: true,
                                payload: token,
                            }),
                            Err(e) => HttpResponse::Ok().json(ErrorResponse {
                                status: false,
                                message: e.to_string(),
                            }),
                        }
                    } else {
                        HttpResponse::NotFound().json(ErrorResponse {
                            status: false,
                            message: "User Not Found.".to_string(),
                        })
                    }
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
