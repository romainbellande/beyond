use crate::config::Config;
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    pub sub: String, // Optional. Subject (whom token refers to)
    pub iat: usize,  // Optional. Issued at (as UTC timestamp)
    pub exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
}

impl Claims {
    pub fn new(sub: String) -> Self {
        let config = Config::new();

        Self {
            sub,
            iat: Utc::now().timestamp() as usize,
            exp: config.jwt_exp,
        }
    }
}

pub struct JwtService {
    secret: String,
}

impl JwtService {
    pub fn new() -> Self {
        let config = Config::new();

        Self {
            secret: config.jwt_secret,
        }
    }

    pub fn create_jwt(&self, sub: String) -> Result<Jwt, jsonwebtoken::errors::Error> {
        let claims = Claims::new(sub);
        let jwt = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.to_string().as_ref()),
        )?;
        Ok(Jwt {
            exp: claims.exp,
            jwt,
        })
    }

    pub fn validate_jwt(
        &self,
        token: String,
    ) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        decode::<Claims>(
            &token,
            &DecodingKey::from_secret(self.secret.to_string().as_ref()),
            &Validation::default(),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Jwt {
    exp: usize,
    jwt: String,
}
