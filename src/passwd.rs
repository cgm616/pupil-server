use argon2rs::verifier::Encoded;
use argon2rs::{Argon2, Variant};
use rand::{thread_rng, Rng};

const SALT_LENGTH: usize = 64;

pub fn hash_password(username: &str, pass: &str, secret: &str) -> String {
    // uses recommendations from https://www.owasp.org/index.php/Password_Storage_Cheat_Sheet

    let mut salt_array = [0u8; SALT_LENGTH];
    thread_rng().fill_bytes(&mut salt_array);
    let hasher = Argon2::new(10, 1, 4096, Variant::Argon2i).unwrap();
    String::from_utf8(Encoded::new(hasher,
                                   pass.as_bytes(),
                                   &salt_array,
                                   secret.as_bytes(),
                                   username.as_bytes())
            .to_u8())
        .unwrap() // TODO: error stuff!!!
}

pub fn verify_password(hash: &str, pass: &str) -> bool {
    Encoded::from_u8(hash.as_bytes()).unwrap().verify(pass.as_bytes())
}

#[cfg(test)]
mod test {
    #[test]
    fn password_hashing() {
        use super::{hash_password, verify_password};

        let username = "supercoolusername";
        let password = "supersafepassword";
        let secret = "don'ttellasoul";

        let hashed = hash_password(username, password, secret);
        let same = verify_password(hashed.as_str(), password);

        assert!(same);
    }
}
