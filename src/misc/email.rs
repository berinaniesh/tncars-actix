use dotenvy::var;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub fn send_email(to: String, otp) -> bool {
    let email = Message::builder()
    .from(format!("Admin (tncars.pp.ua) <{}>", var("EMAIL").expect("Email addresss must be specified in env file")).parse().unwrap())
    .reply_to(format!("Admin <{}>", var("EMAIL").expect("Email addresss must be specified in env file")).parse().unwrap())
    .to(to.parse().unwrap())
    .subject("Verify your account")
    .header(ContentType::TEXT_PLAIN)
    .body(format!("The OTP to verify your account is {}. The OTP is valid for the next 15 minutes", otp));
    .unwrap();

let creds = Credentials::new(var("EMAIL").unwrap(), var("EMAIL_PASSWORD").unwrap());

let mailer = SmtpTransport::relay(var("EMAIL_SERVER").unwrap().as_str())
    .unwrap()
    .credentials(creds)
    .build();

// Send the email
match mailer.send(&email) {
    Ok(_) => return true,
    Err(_) => return false,
    }
}