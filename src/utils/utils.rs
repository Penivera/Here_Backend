use bcrypt::{DEFAULT_COST, hash};

pub fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap()
}
