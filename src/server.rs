use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::ops::Deref;

use rocket::request;
use rocket::response::{Redirect, NamedFile};
use rocket::http::{Cookie, Cookies};
use rocket::State;
use rocket_contrib::{JSON, Value};

use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use super::model::{SafeUser, UserToken, Login, User, NewUser, Register};
use super::error::{Error, ThresholdKind};
use super::passwd;
use super::database::ConnectionPool;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/dash")]
fn dash(user: Result<SafeUser, Error>,
        cookies: &Cookies)
        -> Result<io::Result<NamedFile>, Redirect> {
    match user {
        Ok(user) => {
            if user.conf {
                Ok(NamedFile::open("static/dash.html"))
            } else {
                Err(Redirect::to("/"))
            }
        }
        Err(err) => Err(Redirect::to("/")),
    }
}

#[post("/login", format = "application/json", data = "<data>")]
fn login(cookies: &Cookies,
         data: JSON<Login>,
         pool: State<ConnectionPool>)
         -> Result<JSON<String>, Error> {
    use super::schema::users;

    let data = data.into_inner();

    let connection = pool.0.get()?;

    let user: User = users::table.filter(users::username.eq(&data.username))
        .first::<User>(connection.deref())?;

    if passwd::verify_password(user.pass.as_str(), data.password.as_str()) {
        if user.conf {
            let token = UserToken::new(user.clone())
                .construct_jwt(env::var("JWT_SECRET").expect("JWT_SECRET not set"));
            cookies.add(Cookie::new("jwt", token));
            Ok(JSON(String::from("dash")))
        } else {
            Err(Error::NotConfirmed(ThresholdKind::Login))
        }
    } else {
        Err(Error::BadUserOrPass)
    }

    // TODO make these errors the right errors
}

#[post("/register", format = "application/json", data = "<data>")]
fn register(cookies: &Cookies,
            data: JSON<Register>,
            pool: State<ConnectionPool>)
            -> Result<JSON<String>, Error> {
    use super::schema::users;

    let connection = pool.0.get()?;
    let data = data.into_inner();

    let secret = env::var("HASH_SECRET").expect("HASH_SECRET not set");
    let secure_pass = passwd::hash_password(data.username.as_str(),
                                            data.password.as_str(),
                                            secret.as_str());

    let new_user = NewUser {
        name: data.name.as_str(),
        email: data.email.as_str(),
        username: data.username.as_str(),
        pass: secure_pass.as_str(),
    };

    diesel::insert(&new_user).into(users::table)
        .execute(connection.deref())?;

    Err(Error::NotConfirmed(ThresholdKind::Register))

    // TODO: send confirmation email
}

#[get("/logout")]
fn logout(cookies: &Cookies) -> Redirect {
    cookies.remove("jwt");
    Redirect::to("/")
}

#[get("/favicon.ico")]
fn favicon() -> io::Result<NamedFile> {
    NamedFile::open("static/favicon.ico")
}

#[get("/static/<file..>")]
fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("./static/static/").join(file)).ok()
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::database::ConnectionPool;
    use super::super::model::{Login, NewUser};
    use super::super::error::{Error, ThresholdKind};
    use super::super::schema::users;

    use std::path::PathBuf;
    use std::io::prelude::*;
    use std::io;
    use std::fs::File;
    use std::error::Error as StdError;

    use rocket;
    use rocket::testing::MockRequest;
    use rocket::http::{Status, Method, Cookie, ContentType};

    use diesel::migrations;
    use diesel::pg::PgConnection;

    use serde_json;

    use dotenv::dotenv;

    fn get_root_dir() -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        println!("{:?}", path);
        path
    }

    fn run_migrations() {
        let connection = PgConnection::establish(env::var("DATABASE_URL").unwrap().as_str())
            .unwrap();

        connection.execute("create table users (
          id serial primary key,
          name varchar not null,
          email varchar not null unique,
          username varchar not null unique,
          pass varchar not null,
          conf boolean not null default 'f'
        )")
            .unwrap();

        connection.execute("\
            INSERT INTO users (name, email, username, pass, conf)
                VALUES ('John Smith', 'jsmith@website.com', 'jsmith', '$argon2i$m=4096,t=10,p=1,\
                    keyid=c2VjcmV0,data=anNtaXRo$elvekjRXU/2NqdkYTxb8T155N1QiXMAYhTWdX+vtyOm+kM81W\
                    27CsdOsMabqYkYaM3qKdhOKZuxS0v8bZojvLg$Mqnr5Isv3B3LzWU8WjNFDSklhOf8sANtS41PHBVJ\
                    tFk', true)")
            .unwrap();

        connection.execute("\
            INSERT INTO users (name, email, username, pass, conf)
                VALUES ('Jane Doe', 'jdoe@website.com', 'jdoe', '$argon2i$m=4096,t=10,p=1,\
                    keyid=c2VjcmV0,data=anNtaXRo$elvekjRXU/2NqdkYTxb8T155N1QiXMAYhTWdX+vtyOm+kM81W\
                    27CsdOsMabqYkYaM3qKdhOKZuxS0v8bZojvLg$Mqnr5Isv3B3LzWU8WjNFDSklhOf8sANtS41PHBVJ\
                    tFk', false)")
            .unwrap();
    }

    fn revert_migrations() {
        let connection = PgConnection::establish(env::var("DATABASE_URL").unwrap().as_str())
            .unwrap();

        connection.execute("drop table users").unwrap();
    }

    #[test]
    fn index() {
        let rocket = rocket::ignite().mount("/", routes![super::index]);
        let mut req = MockRequest::new(Method::Get, "/");
        let mut response = req.dispatch_with(&rocket);

        let mut file_pathbuf = get_root_dir();
        file_pathbuf.push("static/index.html");

        let mut file_str = String::new();

        let mut file = File::open(file_pathbuf.as_path()).unwrap();
        file.read_to_string(&mut file_str).unwrap();

        let body = response.body().and_then(|b| b.into_string());

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(body, Some(file_str));
    }

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
            .header(ContentType::JSON)
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
            .header(ContentType::JSON)
            .body(serde_json::to_string(&login).unwrap());
        let mut response = req.dispatch_with(&rocket);

        let body = response.body().and_then(|b| b.into_string());

        revert_migrations();

        assert_eq!(response.status(), Status::BadRequest);
        assert_eq!(body,
                   Some(serde_json::to_string(Error::NotConfirmed(ThresholdKind::Login)
                           .description())
                       .unwrap()));
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
            .header(ContentType::JSON)
            .body(serde_json::to_string(&register).unwrap());
        let mut response = req.dispatch_with(&rocket);

        let body = response.body().and_then(|b| b.into_string());

        let connection = PgConnection::establish(env::var("DATABASE_URL").unwrap().as_str())
            .unwrap();

        let actual_users: Vec<User> = users::table.load(&connection).unwrap();
        let mut actual_safe_users: Vec<SafeUser> = Vec::with_capacity(3);
        for user in actual_users {
            actual_safe_users.push(SafeUser::from(user));
        }

        let expected_safe_users = vec![SafeUser {
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
                                       }];

        revert_migrations();

        assert_eq!(response.status(), Status::BadRequest);
        assert_eq!(body,
                   Some(serde_json::to_string(Error::NotConfirmed(ThresholdKind::Register)
                           .description())
                       .unwrap()));
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
            .header(ContentType::JSON)
            .body(serde_json::to_string(&register).unwrap());
        let mut response = req.dispatch_with(&rocket);

        let body = response.body().and_then(|b| b.into_string());

        revert_migrations();

        assert_eq!(response.status(), Status::BadRequest);
        assert_eq!(body,
                   Some(serde_json::to_string(Error::EmailTaken.description()).unwrap()));
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
            .header(ContentType::JSON)
            .body(serde_json::to_string(&register).unwrap());
        let mut response = req.dispatch_with(&rocket);

        let body = response.body().and_then(|b| b.into_string());

        revert_migrations();

        assert_eq!(response.status(), Status::BadRequest);
        assert_eq!(body,
                   Some(serde_json::to_string(Error::UserTaken.description()).unwrap()));
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

}
