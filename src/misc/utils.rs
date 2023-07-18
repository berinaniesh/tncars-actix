use rand::distributions::{Alphanumeric, DistString};
use regex::Regex;

pub fn generate_otp() -> String {
    let string = Alphanumeric
        .sample_string(&mut rand::thread_rng(), 7)
        .to_uppercase();
    return string;
}

pub fn generate_verify_url() -> String {
    let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 50);
    return string;
}

pub fn validate_email(email: &String) -> bool {
    let email_regex: Regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    let ans = email_regex.is_match(email);
    return ans;
}

pub fn validate_phone(_phone: String) -> bool {
    // Implement this
    return true;
}
