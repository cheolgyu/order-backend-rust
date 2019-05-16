pub mod jwt;
use crate::errors::ServiceError;
use bcrypt::{hash, DEFAULT_COST};

pub fn hash_password(plain: &str) -> Result<String, ServiceError> {
    // get the hashing cost from the env variable or use default
    let hashing_cost: u32 = match std::env::var("HASH_ROUNDS") {
        Ok(cost) => cost.parse().unwrap_or(DEFAULT_COST),
        _ => DEFAULT_COST,
    };
    println!("{}", &hashing_cost);
    hash(plain, hashing_cost).map_err(|_| ServiceError::InternalServerError)
}
