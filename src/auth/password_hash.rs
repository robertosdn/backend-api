use argon2::{password_hash::{PasswordHasher, SaltString}, Argon2};
use rand_core::OsRng;

pub fn hash(password: &str) -> Result<String, &'static str> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default().hash_password(password.as_bytes(), &salt).map(|value| value.to_string()).map_err(|_| "failed to hash password")
}
