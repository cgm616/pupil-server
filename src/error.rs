use std::{io, fmt, error};
use std::error::Error as StdError;

use diesel::result::Error as DieselError;
use diesel::result::{DatabaseErrorKind, DatabaseErrorInformation};

use rocket::response::{Responder, Response};
use rocket::http::{ContentType, Status};

use r2d2::GetTimeout;

#[derive(Debug)]
pub enum Error {
    UserTaken,
    EmailTaken,
    BadUserOrPass,
    NotConfirmed(ThresholdKind),
    DatabaseError(DieselError),
    PoolError(GetTimeout),
}

#[derive(Debug)]
pub enum ThresholdKind {
    Register,
    Login,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", error::Error::description(self))
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadUserOrPass => "Username and password don't match.",
            Error::UserTaken => "That username already exists. Please choose another.",
            Error::EmailTaken => "An account with that email already exists.",
            Error::NotConfirmed(ref kind) => {
                match *kind {
                    ThresholdKind::Register => {
                        "You are successfully registered! Please check your email and click on \
                         the link we sent to confirm your address."
                    }
                    ThresholdKind::Login => {
                        "Please check your email and confirm your email address before signing in."
                    }
                }
            }
            Error::DatabaseError(_) => "The request failed. Please reload and try again.",
            Error::PoolError(_) => "The request failed. Please reload and try again.",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl<'a> Responder<'a> for Error {
    fn respond(self) -> Result<Response<'a>, Status> {
        Ok(Response::build()
            .status(Status::BadRequest)
            .header(ContentType::Plain) // change to JSON?
            .sized_body(io::Cursor::new(String::from(self.description())))
            .finalize())
    }
}

impl From<DieselError> for Error {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => Error::BadUserOrPass,
            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, info) => {
                match info.constraint_name() {
                    Some("users_email_key") => Error::EmailTaken,
                    Some("users_username_key") => Error::UserTaken,
                    _ => {
                        Error::DatabaseError(DieselError::DatabaseError(
                            DatabaseErrorKind::UniqueViolation, info))
                    }
                }
            }
            _ => Error::DatabaseError(err),
        }
    }
}

impl From<GetTimeout> for Error {
    fn from(err: GetTimeout) -> Self {
        Error::PoolError(err)
    }
}
