use jsonwebtoken::{encode, decode, Header, EncodingKey, Validation, Algorithm, DecodingKey, errors::Error};
use chrono::{Utc, NaiveDateTime, Duration};

#[derive(Serialize, Deserialize)]
pub struct JWT {
    pub iat: NaiveDateTime,
    pub exp: NaiveDateTime,
    pub sub: String,
    pub roles: Vec<String>
}

impl JWT {
    fn is_expired(&self) -> bool {
        Utc::now().naive_utc() >= self.exp
    }

    fn is_claimed_user(&self, claimed_user: String) -> bool {
        self.sub == claimed_user
    }

    fn has_role(&self, role: &str) -> bool {
        self.roles.contains(&role.to_string())
    }

    pub fn create(username: String, roles: Vec<String>) -> Result<String, Error> {
        let secret = b"changeme"; // TODO: conf
        let now = Utc::now().naive_utc();
        let claims = JWT {
            iat: now,
            exp: now + Duration::days(7),
            sub: username,
            roles
        };
        // TODO: Algorithm in conf
        encode(&Header::new(Algorithm::ES256), &claims, &EncodingKey::from_secret(secret))
    }

    pub fn decode(token: String) -> Result<JWT, Error> {
        let secret = b"changeme"; // TODO: conf

        // TODO: Algorithm in conf
        match decode::<JWT>(&token, &DecodingKey::from_secret(secret), &Validation::new(Algorithm::ES256)) {
            Ok(token_data) => Ok(token_data.claims),
            Err(err) => Err(err)
        }
    }
}