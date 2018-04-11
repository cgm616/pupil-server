use std::{error, fmt, io};
use std::error::Error;

use diesel::result::Error as DieselError;

use lettre::transport::smtp::error::Error as SmtpError;
use lettre::email::error::Error as EmailError;

use r2d2;

/// An Error type used when the server suffers some error.
/// Implements `std::error::Error`.
#[derive(Debug)]
pub enum ServerError {
    DatabaseError(DieselError),
    PoolError(r2d2::GetTimeout),
    SmtpError(SmtpError),
    EmailError(EmailError),
    EnvVar,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", error::Error::description(self))
    }
}

impl error::Error for ServerError {
    fn description(&self) -> &str {
        match self {
            &ServerError::DatabaseError(_) => "The request failed. Please reload and try again.",
            &ServerError::PoolError(_) => "The request failed. Please reload and try again.",
            &ServerError::SmtpError(_) => "The request failed. Please reload and try again.",
            &ServerError::EmailError(_) => "The request failed. Please reload and try again.",
            &ServerError::EnvVar => "The request failed. Please reload and try again.",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
