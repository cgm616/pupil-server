use chrono::prelude::*;

use super::super::schema::conf;

/// A confirmation/reset struct insertable into the database
#[derive(Insertable)]
#[table_name = "conf"]
pub struct NewConf<'a> {
    pub created: DateTime<Utc>,
    pub userid: i32,
    pub username: &'a str,
    pub link: &'a str,
}

/// A confirmation/reset struct queryable from the database
#[derive(Queryable, Clone, Debug)]
pub struct Conf {
    pub id: i32,
    pub created: DateTime<Utc>,
    pub userid: i32,
    pub username: String,
    pub link: String,
}
