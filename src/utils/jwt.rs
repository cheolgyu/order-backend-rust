use crate::errors::ServiceError;
use crate::svc::auth::model::{AuthUser, SlimUser};
use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, Header, Validation};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    // issuer
    iss: String,
    // account_id
    id: String,
    //role
    role: String,
    // expiry
    exp: i64,
}

// struct to get converted to token and back
impl Claims {
    fn with_user(id: &str, role: &str) -> Self {
        Claims {
            iss: "0.0.0.0:3000".into(),
            id: id.to_owned(),
            role: role.to_owned(),
            exp: (Local::now() + Duration::hours(24)).timestamp(),
        }
    }
}

impl From<Claims> for AuthUser {
    fn from(claims: Claims) -> Self {
        AuthUser {
            id: Uuid::parse_str(claims.id.as_str()).unwrap(),
            role: claims.role,
        }
    }
}

pub fn create_token(data: &SlimUser) -> Result<String, ServiceError> {
    let claims = Claims::with_user(data.id.simple().to_string().as_str(), data.role.as_str());
    encode(&Header::default(), &claims, get_secret().as_ref())
        .map_err(|_err| ServiceError::InternalServerError)
}

pub fn decode_token(token: &str) -> Result<AuthUser, ServiceError> {
    decode::<Claims>(token, get_secret().as_ref(), &Validation::default())
        .map(|data| Ok(data.claims.into()))
        .map_err(|_err| ServiceError::Unauthorized)?
}

fn get_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "my secret".into())
}
