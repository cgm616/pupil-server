use super::super::schema::users;

/// A user data struct insertable into the database.
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub username: &'a str,
    pub pass: &'a str,
}

/// A user data struct queryable from the database.
#[derive(Queryable, Clone, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub username: String,
    pub pass: String,
    pub conf: bool,
}

impl User {
    /// Create the first mock user for testing, with an id of 1
    fn mock_john() -> Self {
        User {
            id: 1,
            name: String::from("John Smith"),
            email: String::from("jsmith@website.com"),
            username: String::from("jsmith"),
            pass: String::from("$argon2i$m=4096,t=10,p=1,keyid=c2VjcmV0,data=an\
                NtaXRo$elvekjRXU/2NqdkYTxb8T155N1QiXMAYhTWdX+vtyOm+kM81W27CsdOs\
                MabqYkYaM3qKdhOKZuxS0v8bZojvLg$Mqnr5Isv3B3LzWU8WjNFDSklhOf8sANt\
                S41PHBVJtFk"),
            conf: true,
        }
    }

    /// Create the second mock user for testing, with an id of 2
    fn mock_jane() -> Self {
        User {
            id: 2,
            name: String::from("Jane Doe"),
            email: String::from("jdoe@website.com"),
            username: String::from("jdoe"),
            pass: String::from("$argon2i$m=4096,t=10,p=1,keyid=c2VjcmV0,data=an\
                NtaXRo$elvekjRXU/2NqdkYTxb8T155N1QiXMAYhTWdX+vtyOm+kM81W27CsdOs\
                MabqYkYaM3qKdhOKZuxS0v8bZojvLg$Mqnr5Isv3B3LzWU8WjNFDSklhOf8sANt\
                S41PHBVJtFk"),
            conf: false,
        }
    }
}