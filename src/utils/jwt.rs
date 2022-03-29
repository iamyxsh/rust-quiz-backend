use std::result::Result;

use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::Error, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

pub fn gen_token(id: String) -> Result<String, Error> {
    let claim = Claims {
        sub: id,
        exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
    };
    let token = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(dotenv::var("JWT_SECRET").unwrap().as_ref()),
    );
    token
}

pub fn get_id(token: String) -> Result<TokenData<Claims>, Error> {
    let token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(dotenv::var("JWT_SECRET").unwrap().as_ref()),
        &Validation::default(),
    );
    token
}
