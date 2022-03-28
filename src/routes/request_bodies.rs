use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct SignupBody {
    #[validate(length(min = 4, max = 12))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 4, max = 12))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct SigninBody {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 4, max = 12))]
    pub password: String,
}
