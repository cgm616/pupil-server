use std::{env, ops::Deref};

use r2d2;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use rocket::{
    http::Status,
    request::{self, FromRequest},
    Request,
    State,
    Outcome
};
use failure::ResultExt;

use super::fail::{PupilError, PupilErrorKind};

/// A type that implements `rocket::request::FromRequest` to work as a request guard in handlers. 
/// Gives a handler access to a DB connection.
pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

/// A type alias to make it easier to refer to the r2d2 pool.
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;


/// Constructs a new Pool for use as a managed state.
pub fn new_pool() -> Result<Pool, PupilError> {
    let manager = ConnectionManager::<PgConnection>::new(
        env::var("DATABASE_URL").context(PupilErrorKind::EnvVar)?,
    );
        
    Ok(r2d2::Pool::builder().build(manager).context(PupilErrorKind::PoolError)?)
}

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.inner().get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// For the convenience of using an &DbConn as an &SqliteConnection.
impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
