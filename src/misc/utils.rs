use rand::distributions::{Alphanumeric, DistString};

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

pub fn validate_email(email: String) -> bool {
    return true;
}

pub fn validate_phone(phone: String) -> bool {
    return true;
}
