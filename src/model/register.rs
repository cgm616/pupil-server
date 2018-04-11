use rocket::{Data, Request};
use rocket::data;
use rocket::http::{ContentType, Status};
use rocket::Outcome;
use rocket_contrib::Json;
use psl;
use failure::Fail;

use fail::{FieldError, PupilError, PupilErrorKind, ThresholdField};

/// A register data struct deserializeabe from Json data sent by the client.
#[derive(Serialize, Deserialize)]
pub struct Register {
    pub name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

impl data::FromData for Register {
    type Error = PupilError;

    fn from_data(request: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        // ensure the content type is json
        if request.content_type() != Some(&ContentType::new("application", "json")) {
            return Outcome::Forward(data);
        }

        // deserialize the data from Json using FromData impl from rocket_contrib
        let outcome: data::Outcome<Json<Register>, _> = Json::from_data(request, data);

        // return Outcome variant based on what Json's FromData gave,
        // validating if successful, forwarding if forward, and converting to Error if failure
        match outcome {
            Outcome::Success(deserialized) => {
                // get Register struct from Json
                let data: Register = deserialized.into_inner();

                // validate the registration data according to the standards:
                // https://github.com/cgm616/pupil-server/issues/19#issuecomment-312779376
                let name: bool = 4 <= data.name.len() && data.name.len() <= 128;
                let email: bool =
                    5 <= data.email.len() && data.email.len() <= 128 && valid_email(&data.email);
                let username: bool = 3 <= data.username.len() && data.username.len() <= 32
                    && valid_username(&data.username);
                let password: bool = 8 <= data.password.len() && data.password.len() <= 128;

                // check each possibility of invalidity and return it
                if !name {
                    Outcome::Failure((
                        Status::BadRequest,
                        PupilErrorKind::AuthInput {
                            field: ThresholdField::Name,
                            error: FieldError::Invalid,
                        }.into(),
                    ))
                } else if !email {
                    Outcome::Failure((
                        Status::BadRequest,
                        PupilErrorKind::AuthInput {
                            field: ThresholdField::Email,
                            error: FieldError::Invalid,
                        }.into(),
                    ))
                } else if !username {
                    Outcome::Failure((
                        Status::BadRequest,
                        PupilErrorKind::AuthInput {
                            field: ThresholdField::User,
                            error: FieldError::Invalid,
                        }.into(),
                    ))
                } else if !password {
                    Outcome::Failure((
                        Status::BadRequest,
                        PupilErrorKind::AuthInput {
                            field: ThresholdField::Pass,
                            error: FieldError::Invalid,
                        }.into(),
                    ))
                } else {
                    // return validated register struct
                    Outcome::Success(data)
                }
            }
            Outcome::Failure((status, error)) => {
                Outcome::Failure((status, error.context(PupilErrorKind::BadJson).into()))
            }
            Outcome::Forward(data) => Outcome::Forward(data),
        }
    }
}

/// Check to see if an email address is a valid email using the `publicsuffix` library.
fn valid_email(email: &str) -> bool {
    match psl::get().parse_email(email) {
        Err(_) => false, // if an error parsing email, return false for an invalid email
        Ok(_) => true,   // if no errors, email is valid
    }
}

/// Check to see if a username is valid according to the rules in
/// https://github.com/cgm616/pupil-server/issues/19#issuecomment-312779376
fn valid_username(username: &str) -> bool {
    true
}