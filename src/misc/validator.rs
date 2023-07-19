use chrono::{Datelike, Utc};
use regex::Regex;

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

pub fn validate_year(year: i32) -> i32 {
    let current_time = Utc::now();
    let current_year = current_time.year();
    // Modern cars invented in 1886
    if year >= 1886 && year <= current_year {
        return year;
    }
    if year < 1886 {
        return 1886;
    } else {
        return current_year;
    }
}
