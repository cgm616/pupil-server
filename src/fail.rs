use failure::Context;
use std::{fmt, io};
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket::http::{ContentType, Status};
use serde_json;

#[derive(Debug)]
pub struct PupilError {
    inner: Context<PupilErrorKind>,
}

#[derive(Debug, Fail, Eq, PartialEq, Copy, Clone, Serialize)]
pub enum PupilErrorKind {
    DatabaseError,
    PoolError,
    EmailError,
    EnvVar,
    AuthInput {
        field: ThresholdField,
        error: FieldError,
    },
    BadJson,
    BadCookie,
}

/// Denotes which field of request input was bad in a `RequestError::AuthInput`.
#[derive(Debug, Eq, Copy, Clone, PartialEq, Serialize)]
pub enum ThresholdField {
    User,
    Pass,
    Name,
    Email,
    UserOrPass,
}

/// Denotes the specific problem with the field in a `RequestError::AuthInput`.
#[derive(Debug, Eq, Copy, Clone, PartialEq, Serialize)]
pub enum FieldError {
    Taken,
    Invalid,
}

impl PupilError {
    pub fn kind(&self) -> PupilErrorKind {
        *self.inner.get_context()
    }
}

impl From<PupilErrorKind> for PupilError {
    fn from(kind: PupilErrorKind) -> PupilError {
        PupilError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<PupilErrorKind>> for PupilError {
    fn from(inner: Context<PupilErrorKind>) -> PupilError {
        PupilError { inner: inner }
    }
}

impl fmt::Display for PupilErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match *self {
            PupilErrorKind::AuthInput {
                ref field,
                ref error,
            } => match (field, error) {
                (&ThresholdField::User, &FieldError::Taken) => "That username is already taken.",
                (&ThresholdField::User, &FieldError::Invalid) => {
                    "That username is invalid. Usernames can only include alphanumeric \
                     characters and underscores and must be between 3 and 32 characters."
                }

                (&ThresholdField::Pass, &FieldError::Taken) => "",
                (&ThresholdField::Pass, &FieldError::Invalid) => {
                    "Passwords must be between 8 and 128 characters."
                }

                (&ThresholdField::Name, &FieldError::Taken) => "",
                (&ThresholdField::Name, &FieldError::Invalid) => {
                    "Names must be between 4 and 128 characters."
                }

                (&ThresholdField::Email, &FieldError::Taken) => {
                    "That email is already assigned to another account. If that is your \
                     account, please login instead of creating a new account."
                }
                (&ThresholdField::Email, &FieldError::Invalid) => {
                    "That email is not a valid email."
                }

                (&ThresholdField::UserOrPass, &FieldError::Taken) => "",
                (&ThresholdField::UserOrPass, &FieldError::Invalid) => {
                    "Either the username or password is invalid. Please try again."
                }
            },
            PupilErrorKind::BadCookie => "Your authentication cookie has expired.",
            PupilErrorKind::BadJson => "The request JSON was invalid.",
            _ => "The request failed. Please try again.",
        };

        write!(f, "{}", output)
    }
}

impl<'a> Responder<'a> for PupilError {
    fn respond_to(self, _req: &Request) -> Result<Response<'a>, Status> {
        // I don't want all to return a message, some should redirect and so forth

        match self.kind() {
            PupilErrorKind::AuthInput{ .. } => {
                let body = serde_json::to_string(&self.kind()).map_err(|_err| Status::BadRequest)?;
                
                Ok(Response::build()
                    .status(Status::BadRequest) // fix status codes too, maybe .cause()???
                    .header(ContentType::JSON)
                    .sized_body(io::Cursor::new(body))
                    .finalize())
            },
            _  => { unimplemented!() }
        }

        
    }
}
