pub mod client;
pub mod jwt;
pub mod validator;

lazy_static! {
    static ref HASH_ROUNDS: String = std::env::var("HASH_ROUNDS").unwrap();
}
/*
pub fn hash_password(plain: &str) -> Result<String, ServiceError> {
    // get the hashing cost from the env variable or use default
    /*
    let hashing_cost: u32 = match HASH_ROUNDS {
        Ok(cost) => cost.parse().unwrap_or(DEFAULT_COST),
        _ => DEFAULT_COST,
    };
    */
    let hashing_cost: u32 = HASH_ROUNDS.parse().unwrap_or(DEFAULT_COST);
    println!("{}", &hashing_cost);
    hash(plain, hashing_cost).map_err(|_| ServiceError::InternalServerError)
}
*/
