use std::thread;
use std::env;

use r2d2;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

pub struct ConnectionPool(pub r2d2::Pool<ConnectionManager<PgConnection>>);

impl ConnectionPool {
    pub fn new() -> Self {
        let config = r2d2::Config::default();
        let manager = ConnectionManager::<PgConnection>::new(env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set"));
        ConnectionPool(r2d2::Pool::new(config, manager).expect("Failed to create pool."))
    }
}
