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
pub fn verify(naive_password: &String, hashed_password: &String) -> bool {
    let argon2: Argon2 = Argon2::default();
    let naive_pw = naive_password.as_bytes();
    let actual_hashed_pw = PasswordHash::new(&hashed_password).unwrap();
    let ans = argon2
        .verify_password(naive_pw, &actual_hashed_pw)
        .is_ok();
    return ans;
}
