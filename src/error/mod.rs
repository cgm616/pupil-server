use std::{io, fmt, error};
use std::error::Error as StdError;
use std::env::VarError;

use diesel::result::Error as DieselError;
use diesel::result::{DatabaseErrorKind, DatabaseErrorInformation};

use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket::http::{ContentType, Status};
use rocket_contrib::json::SerdeError;

use lettre::transport::smtp::error::Error as SmtpError;
use lettre::email::error::Error as EmailError;

use serde_json;
use r2d2;

pub mod server;
pub mod request;

pub use error::server::ServerError;
pub use error::request::{RequestError, ThresholdField, FieldError};

/// The Error struct implements `std::error::Error`, `From` for a number of dependency errors, and `rocket::response::Responder`.
/// Each branch contains extra information, with types with their own `Error` implementations.
#[derive(Debug)]
pub enum Error {
    Server(ServerError),
    Request(RequestError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", error::Error::description(self))
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::Server(ref err) => err.description(),
            &Error::Request(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl From<ServerError> for Error {
    fn from(err: ServerError) -> Self {
        Error::Server(err)
    }
}

impl From<RequestError> for Error {
    fn from(err: RequestError) -> Self {
        Error::Request(err)
    }
}

impl<'a> Responder<'a> for Error {
    fn respond_to(self, req: &Request) -> Result<Response<'a>, Status> {
        // I don't want all to return a message, some should redirect and so forth

        let body =
            io::Cursor::new(serde_json::to_string(self.description()) //fix something wth this json
            .unwrap_or(String::from(self.description())));

        Ok(Response::build()
            .status(Status::BadRequest) // fix status codes too, maybe .cause()???
            .header(ContentType::JSON)
            .sized_body(body)
            .finalize())
    }
}

impl From<DieselError> for Error {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => {
                Error::Request(RequestError::AuthInput(ThresholdField::UserOrPass,
                                                       FieldError::Invalid))
            }
            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, info) => {
                let message = info.message().to_owned();
                if message.contains("users_email_key") {
                    Error::Request(RequestError::AuthInput(ThresholdField::Email,
                                                           FieldError::Taken))
                } else if message.contains("users_username_key") {
                    Error::Request(RequestError::AuthInput(ThresholdField::User, FieldError::Taken))
                } else {
                    Error::Server(ServerError::DatabaseError(DieselError::DatabaseError(
                        DatabaseErrorKind::UniqueViolation, info)))
                }
            }
            _ => Error::Server(ServerError::DatabaseError(err)),
        }
    }
}

impl From<r2d2::GetTimeout> for Error {
    fn from(err: r2d2::GetTimeout) -> Self {
        Error::Server(ServerError::PoolError(err))
    }
}

impl From<VarError> for Error {
    fn from(err: VarError) -> Self {
        Error::Server(ServerError::EnvVar)
    }
}

impl From<SerdeError> for Error {
    fn from(err: SerdeError) -> Self {
        Error::Request(RequestError::BadJson(err))
    }
}

impl From<SmtpError> for Error {
    fn from(err: SmtpError) -> Self {
        println!("{:?}", &err);
        Error::Server(ServerError::SmtpError(err))
    }
}

impl From<EmailError> for Error {
    fn from(err: EmailError) -> Self {
        Error::Server(ServerError::EmailError(err))
    }
}
