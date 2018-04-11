use std::env;
use std::io;
use std::path::{Path, PathBuf};
use rand::{thread_rng, Rng};

use rocket::response::{NamedFile, Redirect};
use rocket::http::{Cookie, Cookies};
use rocket::State;
use rocket_contrib::Json;
use diesel;
use diesel::prelude::*;
use chrono::prelude::*;
use lettre::{EmailAddress, EmailTransport, SimpleSendableEmail};
use crockford;
use failure::{Fail, ResultExt};

use model::{Conf, Login, NewConf, NewUser, Register, SafeUser, User, UserToken};
use fail::{FieldError, PupilError, PupilErrorKind, ThresholdField};
use passwd;
use database::DbConn;
use super::Mailer;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/dash")]
fn dash(user: Result<SafeUser, PupilError>) -> Result<io::Result<NamedFile>, Redirect> {
    match user {
        Ok(_) => Ok(NamedFile::open("static/dash.html")),
        Err(_) => Err(Redirect::to("/")),
    }
}

#[post("/login", format = "application/json", data = "<data>")]
fn login(
    mut cookies: Cookies,
    data: Json<Login>,
    connection: DbConn,
) -> Result<Redirect, PupilError> {
    use super::schema::users;

    // get Login struct from Json
    let data = data.into_inner();

    // grab the User struct with the same username from the database
    let user: User = users::table
        .filter(users::username.eq(&data.username))
        .first::<User>(&*connection)
        .context(PupilErrorKind::DatabaseError)?;

    // check if the passwords match
    if passwd::verify_password(user.pass.as_str(), data.password.as_str()) {
        // check if the user has confirmed their email if passwords match
        if user.conf {
            // if they have, create a new auth token and add it to the cookies
            let token = UserToken::new(user.clone())
                .construct_jwt(env::var("JWT_SECRET").context(PupilErrorKind::EnvVar)?);
            cookies.add(Cookie::new("jwt", token));

            // then, redirect the user to their dashboard
            Ok(Redirect::to("/dash"))
        } else {
            // if they haven't, don't set any cookies (they can't sign in yet) and redirect to confirm page
            Ok(Redirect::to("/confirm"))
        }
    } else {
        // if they don't match, then the password was wrong and return a request error saying so
        Err(PupilErrorKind::AuthInput {
            field: ThresholdField::UserOrPass,
            error: FieldError::Invalid,
        })?
    }
}

#[post("/register", format = "application/json", data = "<data>")]
fn register(
    data: Register,
    connection: DbConn,
    mailer: State<Mailer>,
) -> Result<Redirect, PupilError> {
    use super::schema::{conf, users};

    // hash the password with a random salt (see hash_password function)
    let secret = env::var("HASH_SECRET").context(PupilErrorKind::EnvVar)?;
    let secure_pass = passwd::hash_password(
        data.username.as_str(),
        data.password.as_str(),
        secret.as_str(),
    );

    // create a new user with the registration data
    let new_user = NewUser {
        name: data.name.as_str(),
        email: data.email.as_str(),
        username: data.username.as_str(),
        pass: secure_pass.as_str(),
    };

    // insert the new user into the database
    let user: User = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&*connection)
        .context(PupilErrorKind::DatabaseError)?;

    let random = thread_rng().gen::<u64>();
    let encoded = crockford::encode(random);

    let new_conf = NewConf {
        created: Utc::now(),
        userid: user.id,
        username: user.username.as_str(),
        link: &encoded,
    };

    diesel::insert_into(conf::table)
        .values(&new_conf)
        .execute(&*connection)
        .context(PupilErrorKind::DatabaseError)?;

    let email = SimpleSendableEmail::new(EmailAddress::new("founders@usepupil.us".to_string()), vec![EmailAddress::new(data.email.clone())], "message_id".to_string(), format!(
            "<p>Thank-you for joining Pupil! To confirm your email address, please click the link below.</p>

            <a href=\"{}\">Confirm email address</a> ({})

            <p>Thanks again,<p>

            <p>Cole Graber-Mitchell and Madhav Singh, <\\ br>
            Founders of Pupil</p>
            ", &encoded, &encoded));

    mailer
        .lock()
        .expect("Mutex was poisoned!")
        .send(&email)
        .context(PupilErrorKind::EmailError)?;

    Ok(Redirect::to("/confirm"))
}

/// Handler to show the confirmation page
#[get("/confirm")]
fn not_confirmed() -> io::Result<NamedFile> {
    NamedFile::open("static/confirm.html")
}

/// Handler to confirm accounts
#[get("/confirm/<key>")]
fn confirm(key: String) -> io::Result<NamedFile> {
    let key_matches = true; // This needs to be a database lookup

    if key_matches {
        // If they have the right key, confirm the account and ask to log in
        NamedFile::open("static/confirmed.html")
    } else {
        // If they don't, do _something_. Show 404 page?
        unimplemented!()
    }
}

/// Handler to log users out
#[get("/logout")]
fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove(Cookie::new("jwt", "invalidtoken")); // remove the json web token cookie
    Redirect::to("/") // then redirect back to the home page
}

/// Handler to allow browsers to get the favicon image
#[get("/favicon.ico")]
fn favicon() -> io::Result<NamedFile> {
    NamedFile::open("static/favicon.ico")
}

/// Generic handler to serve up any file in the static/static/ folder,
/// which is generated by the frontend code
#[get("/static/<file..>")]
fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("./static/static/").join(file)).ok()
}

#[cfg(test)]
mod test {
    use super::super::embedded_migrations;
    use super::super::init_rocket;

    use std::{env, fs, path, io::{self, Read}};

    use diesel::pg::PgConnection;
    use diesel::prelude::*;
    use serde_json;
    use dotenv::dotenv;
    use rocket::local::Client;
    use rocket::http::Status;

    fn get_file(donde: &str) -> String {
        let mut file_string = String::new();

        let mut file_path = path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file_path.push(donde);

        let mut file = fs::File::open(file_path.as_path()).unwrap();
        file.read_to_string(&mut file_string).unwrap();

        file_string
    }

    fn run_migrations() {
        let connection =
            PgConnection::establish(env::var("DATABASE_URL").unwrap().as_str()).unwrap();

        connection
            .execute(
                "create table users (
          id serial primary key,
          name varchar not null,
          email varchar not null unique,
          username varchar not null unique,
          pass varchar not null,
          conf boolean not null default 'f'
        )",
            )
            .unwrap();

        connection
            .execute(
                "\
            INSERT INTO users (name, email, username, pass, conf)
                VALUES ('John Smith', 'jsmith@website.com', 'jsmith', '$argon2i$m=4096,t=10,p=1,\
                    keyid=c2VjcmV0,data=anNtaXRo$elvekjRXU/2NqdkYTxb8T155N1QiXMAYhTWdX+vtyOm+kM81W\
                    27CsdOsMabqYkYaM3qKdhOKZuxS0v8bZojvLg$Mqnr5Isv3B3LzWU8WjNFDSklhOf8sANtS41PHBVJ\
                    tFk', true)",
            )
            .unwrap();

        connection
            .execute(
                "\
            INSERT INTO users (name, email, username, pass, conf)
                VALUES ('Jane Doe', 'jdoe@website.com', 'jdoe', '$argon2i$m=4096,t=10,p=1,\
                    keyid=c2VjcmV0,data=anNtaXRo$elvekjRXU/2NqdkYTxb8T155N1QiXMAYhTWdX+vtyOm+kM81W\
                    27CsdOsMabqYkYaM3qKdhOKZuxS0v8bZojvLg$Mqnr5Isv3B3LzWU8WjNFDSklhOf8sANtS41PHBVJ\
                    tFk', false)",
            )
            .unwrap();
    }

    fn establish_connection() -> PgConnection {
        PgConnection::establish(env::var("DATABASE_URL").unwrap().as_str()).unwrap()
    }

    #[test]
    fn migrations() {
        dotenv().ok();

        let connection = establish_connection();

        embedded_migrations::run_with_output(&connection, &mut io::stdout()).unwrap();
    }

    /*
    fn revert_migrations() {
        let connection =
            PgConnection::establish(env::var("DATABASE_URL").unwrap().as_str()).unwrap();

        connection.execute("drop table users").unwrap();
    }
*/

    #[test]
    fn index() {
        dotenv().ok();

        let client = Client::new(init_rocket().unwrap()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();

        let file_string = get_file("static/index.html");

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(file_string));
    }

    #[test]
    fn login_good_conf() {
        unimplemented!();
    }

    #[test]
    fn login_good_unconf() {
        unimplemented!();
    }

    #[test]
    fn login_bad() {
        unimplemented!();
    }

    #[test]
    fn conf_good() {
        unimplemented!();
    }

    #[test]
    fn conf_bad() {
        unimplemented!();
    }

    #[test]
    fn dashboard_auth() {
        unimplemented!();
    }

    #[test]
    fn dashboard_unauth() {
        unimplemented!();
    }

    #[test]
    fn register_good() {
        unimplemented!();
    }

    #[test]
    fn register_bad() {
        unimplemented!();
    }

    #[test]
    fn register_error() {
        unimplemented!();
    }

    /*
    #[test]
    fn dash_authed() {
        dotenv().ok();

        let name = String::from("John Smith");
        let email = String::from("jsmith@website.com");
        let username = String::from("jsmith");
        let pass = String::from("hashed_password");
        let conf = true;

        let mut claims = UserToken::new(User {
            id: 1,
            name: name,
            email: email,
            username: username,
            pass: pass,
            conf: conf,
        });

        let token = claims.construct_jwt(env::var("JWT_SECRET").unwrap());

        let rocket = rocket::ignite().mount("/", routes![super::dash]);
        let mut req = MockRequest::new(Method::Get, "/dash").cookie(Cookie::new("jwt", token));
        let mut response = req.dispatch_with(&rocket);

        let mut file_pathbuf = get_root_dir();
        file_pathbuf.push("static/dash.html");

        let mut file_str = String::new();

        let mut file = File::open(file_pathbuf.as_path()).unwrap();
        file.read_to_string(&mut file_str).unwrap();

        let body = response.body().and_then(|b| b.into_string());

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(body, Some(file_str));
    }

    #[test]
    fn dash_authed_not_confirmed() {
        dotenv().ok();

        let name = String::from("John Smith");
        let email = String::from("jsmith@website.com");
        let username = String::from("jsmith");
        let pass = String::from("hashed_password");
        let conf = false;

        let mut claims = UserToken::new(User {
            id: 1,
            name: name,
            email: email,
            username: username,
            pass: pass,
            conf: conf,
        });

        let token = claims.construct_jwt(env::var("JWT_SECRET").unwrap());

        let rocket = rocket::ignite().mount("/", routes![super::dash]);
        let mut req = MockRequest::new(Method::Get, "/dash").cookie(Cookie::new("jwt", token));
        let mut response = req.dispatch_with(&rocket);

        assert_eq!(response.status(), Status::SeeOther);
    }

    #[test]
    fn dash_unauthed() {
        let rocket = rocket::ignite().mount("/", routes![super::dash]);
        let mut req = MockRequest::new(Method::Get, "/dash");
        let mut response = req.dispatch_with(&rocket);

        assert_eq!(response.status(), Status::SeeOther);
    }

    #[test]
    fn login_confirmed() {
        dotenv().ok();

        run_migrations();

        let login = Login {
            username: String::from("jsmith"),
            password: String::from("test"),
        };

        let rocket = rocket::ignite()
            .manage(ConnectionPool::new())
            .mount("/", routes![super::login]);
        let mut req = MockRequest::new(Method::Post, "/login")
            .header(ContentType::Json)
            .body(serde_json::to_string(&login).unwrap());
        let mut response = req.dispatch_with(&rocket);

        let body = response.body().and_then(|b| b.into_string());

        revert_migrations();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(body, Some(serde_json::to_string(&"dash").unwrap()));
    }

    #[test]
    fn login_not_confirmed() {
        dotenv().ok();

        run_migrations();

        let login = Login {
            username: String::from("jdoe"),
            password: String::from("test"),
        };

        let rocket = rocket::ignite()
            .manage(ConnectionPool::new())
            .mount("/", routes![super::login]);
        let mut req = MockRequest::new(Method::Post, "/login")
            .header(ContentType::Json)
            .body(serde_json::to_string(&login).unwrap());
        let mut response = req.dispatch_with(&rocket);

        let body = response.body().and_then(|b| b.into_string());

        revert_migrations();

        assert_eq!(response.status(), Status::BadRequest);
        assert_eq!(
            body,
            Some(
                serde_json::to_string(Error::NotConfirmed(ThresholdKind::Login).description())
                    .unwrap(),
            )
        );
    }

    #[test]
    fn register_new() {
        dotenv().ok();

        run_migrations();

        let new_name = "Diff Perse";
        let new_email = "dperse@website.com";
        let new_username = "dperse";

        let register = Register {
            name: String::from(new_name),
            email: String::from(new_email),
            username: String::from(new_username),
            password: String::from("bad_pass"),
        };

        let rocket = rocket::ignite()
            .manage(ConnectionPool::new())
            .mount("/", routes![super::register]);
        let mut req = MockRequest::new(Method::Post, "/register")
            .header(ContentType::Json)
            .body(serde_json::to_string(&register).unwrap());
        let mut response = req.dispatch_with(&rocket);

        let body = response.body().and_then(|b| b.into_string());

        let connection =
            PgConnection::establish(env::var("DATABASE_URL").unwrap().as_str()).unwrap();

        let actual_users: Vec<User> = users::table.load(&connection).unwrap();
        let mut actual_safe_users: Vec<SafeUser> = Vec::with_capacity(3);
        for user in actual_users {
            actual_safe_users.push(SafeUser::from(user));
        }

        let expected_safe_users = vec![
            SafeUser {
                id: 1,
                name: String::from("John Smith"),
                email: String::from("jsmith@website.com"),
                username: String::from("jsmith"),
                conf: true,
            },
            SafeUser {
                id: 2,
                name: String::from("Jane Doe"),
                email: String::from("jdoe@website.com"),
                username: String::from("jdoe"),
                conf: false,
            },
            SafeUser {
                id: 3,
                name: String::from(new_name),
                email: String::from(new_email),
                username: String::from(new_username),
                conf: false,
            },
        ];

        revert_migrations();

        assert_eq!(response.status(), Status::BadRequest);
        assert_eq!(
            body,
            Some(
                serde_json::to_string(Error::NotConfirmed(ThresholdKind::Register).description())
                    .unwrap(),
            )
        );
        assert_eq!(expected_safe_users, actual_safe_users);
    }

    #[test]
    fn register_email_existing() {
        dotenv().ok();

        run_migrations();

        let register = Register {
            name: String::from("Jane Doe"),
            email: String::from("jdoe@website.com"),
            username: String::from("jdoe2"),
            password: String::from("bad_pass"),
        };

        let rocket = rocket::ignite()
            .manage(ConnectionPool::new())
            .mount("/", routes![super::register]);
        let mut req = MockRequest::new(Method::Post, "/register")
            .header(ContentType::Json)
            .body(serde_json::to_string(&register).unwrap());
        let mut response = req.dispatch_with(&rocket);

        let body = response.body().and_then(|b| b.into_string());

        revert_migrations();

        assert_eq!(response.status(), Status::BadRequest);
        assert_eq!(
            body,
            Some(serde_json::to_string(Error::EmailTaken.description()).unwrap(),)
        );
    }

    #[test]
    fn register_username_existing() {
        dotenv().ok();

        run_migrations();

        let register = Register {
            name: String::from("Jane Doe"),
            email: String::from("jdoe2@website.com"),
            username: String::from("jdoe"),
            password: String::from("bad_pass"),
        };

        let rocket = rocket::ignite()
            .manage(ConnectionPool::new())
            .mount("/", routes![super::register]);
        let mut req = MockRequest::new(Method::Post, "/register")
            .header(ContentType::Json)
            .body(serde_json::to_string(&register).unwrap());
        let mut response = req.dispatch_with(&rocket);

        let body = response.body().and_then(|b| b.into_string());

        revert_migrations();

        assert_eq!(response.status(), Status::BadRequest);
        assert_eq!(
            body,
            Some(serde_json::to_string(Error::UserTaken.description()).unwrap(),)
        );
    }

    #[test]
    fn favicon() {
        let rocket = rocket::ignite().mount("/", routes![super::favicon]);
        let mut req = MockRequest::new(Method::Get, "/favicon.ico");
        let mut response = req.dispatch_with(&rocket);

        let mut file_pathbuf = get_root_dir();
        file_pathbuf.push("static/favicon.ico");

        let mut file_buffer: Vec<u8> = Vec::new();

        let mut file = File::open(file_pathbuf.as_path()).unwrap();
        file.read_to_end(&mut file_buffer).unwrap();

        let body: Option<Vec<u8>> = response.body().and_then(|b| b.into_bytes());

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(body, Some(file_buffer));
    }
    */

}
