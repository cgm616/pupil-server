use std::{error, fmt};
use std::fmt::Display;
use std::error::Error;

use rocket_contrib::json::SerdeError;

/// An Error type used when the request was somehow bad.
/// Implements `std::error::Error`.
#[derive(Debug)]
pub enum RequestError {
    AuthInput(ThresholdField, FieldError),
    BadJson(SerdeError),
    BadCookie,
}

/// Denotes which field of request input was bad in a `RequestError::AuthInput`.
#[derive(Debug)]
pub enum ThresholdField {
    User,
    Pass,
    Name,
    Email,
    UserOrPass,
}

/// Denotes the specific problem with the field in a `RequestError::AuthInput`.
#[derive(Debug)]
pub enum FieldError {
    Taken,
    Invalid,
}

impl Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", error::Error::description(self))
    }
}

impl Error for RequestError {
    fn description(&self) -> &str {
        match *self {
            RequestError::AuthInput(ref field, ref err) => match (field, err) {
                (&ThresholdField::User, &FieldError::Taken) => "That username is already taken.",
                (&ThresholdField::User, &FieldError::Invalid) => {
                    "That username is invalid. Usernames can only include alphanumeric \
                     characters and underscores and must be between 3 and 32 characters."
                }

                (&ThresholdField::Pass, &FieldError::Taken) => unimplemented!(),
                (&ThresholdField::Pass, &FieldError::Invalid) => {
                    "Passwords must be between 8 and 128 characters."
                }

                (&ThresholdField::Name, &FieldError::Taken) => unimplemented!(),
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

                (&ThresholdField::UserOrPass, &FieldError::Taken) => unimplemented!(),
                (&ThresholdField::UserOrPass, &FieldError::Invalid) => {
                    "Either the username or password is invalid. Please try again."
                }
            },
            RequestError::BadCookie => "Your authentication cookie has expired.",
            RequestError::BadJson(_) => "The request JSON was invalid.",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
