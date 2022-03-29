use pwhash::{bcrypt, error::Error};

pub fn gen_hash(password: String) -> Result<String, Error> {
    bcrypt::hash(password)
}

pub fn verify_hash(hash: String, password: String) -> bool {
    bcrypt::verify(password, hash.as_str())
}
