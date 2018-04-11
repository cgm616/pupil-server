use chrono::prelude::*;
use jwt::{encode, Header};

use super::user::User;

/// A user data struct for use with Json Web Tokens for cookie authentication.
#[derive(Debug, Deserialize, Serialize)]
pub struct UserToken {
    pub iat: i64,
    pub exp: i64,
    pub iss: String,
    pub id: i32,
    pub name: String,
    pub email: String,
    pub username: String,
    pub conf: bool,
}

static ONE_MIN: i64 = 60;
static ISSUER: &'static str = "pupil";

impl UserToken {
    /// Creates a new UserToken for serialization into a JWT.
    pub fn new(user: User) -> Self {
        let now = Utc::now();
        UserToken {
            iat: now.timestamp(),
            exp: now.timestamp() + ONE_MIN,
            iss: String::from(ISSUER),
            id: user.id,
            name: user.name,
            email: user.email,
            username: user.username,
            conf: user.conf,
        }
    }

    /// Serializes a `UserToken` into an encrypted JWT cookie string.
    pub fn construct_jwt(&self, secret: String) -> String {
        encode(&Header::default(), self, secret.as_bytes()).unwrap() // TODO error handling
    }
}

#[cfg(test)]
mod tests {
    use super::super::user::User;
    use super::*;

    #[test]
    fn encode_jwt() {
        let issued_at = 1492907635;
        let expired = 1492907695;
        let name = String::from("John Smith");
        let email = String::from("jsmith@website.com");
        let username = String::from("jsmith");
        let pass = String::from("hashed_password");
        let conf = true;

        let mut claims = UserToken::new(User {
            id: 1,
            name: name,
            email: email,
            username: username,
            pass: pass,
            conf: conf,
        });

        claims.iat = issued_at;
        claims.exp = expired;

        let encoded = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE0OTI5MDc2MzUsImV4cCI6MTQ5Mj\
            kwNzY5NSwiaXNzIjoicHVwaWwiLCJpZCI6MSwibmFtZSI6IkpvaG4gU21pdGgiLCJlbWFpbCI6ImpzbWl0aEB3\
            ZWJzaXRlLmNvbSIsInVzZXJuYW1lIjoianNtaXRoIiwiY29uZiI6dHJ1ZX0.655jzhRXSF05RJyACAWv_tuT9p\
            MMVyIyMh4Icb6EKOI";

        assert_eq!(claims.construct_jwt(String::from("secret")), encoded);
    }
}