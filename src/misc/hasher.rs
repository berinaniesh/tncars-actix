use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

// Add better error handing
pub fn hash(naive_password: &String) -> String {
    let argon2: Argon2 = Argon2::default();
    let password = naive_password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2.hash_password(password, &salt).unwrap().to_string();
    return password_hash;
}

// Add better error handling
pub fn verify(p1: String, p2: String) -> bool {
    let argon2: Argon2 = Argon2::default();
    let naive_password = p1.as_bytes();
    let actual_hashed_password = PasswordHash::new(&p2).unwrap();
    let ans = argon2
        .verify_password(naive_password, &actual_hashed_password)
        .is_ok();
    return ans;
}
