mod conf; // TODO: remove pub on this
mod user;
mod register;
mod safe_user;
mod token;

pub use self::conf::*;
pub use self::user::*;
pub use self::register::*;
pub use self::safe_user::*;
pub use self::token::*;

/// A login data struct deserializeable from Json data sent by the client.
#[derive(Serialize, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}
