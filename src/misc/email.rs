use dotenvy::var;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub fn send_email(to: String, otp: String, verify_url: String, email_subject: String, email_body: String) -> bool {
    let email = Message::builder()
    .from(format!("Admin (tncars.pp.ua) <{}>", var("EMAIL").expect("Email addresss must be specified in env file")).parse().unwrap())
    .reply_to(format!("Admin <{}>", var("EMAIL").expect("Email addresss must be specified in env file")).parse().unwrap())
    .to(to.parse().unwrap())
    .subject(email_subject)
    .header(ContentType::TEXT_PLAIN)
    .body(email_body)
    .unwrap();

    let creds = Credentials::new(var("EMAIL").unwrap(), var("EMAIL_PASSWORD").unwrap());

    let mailer = SmtpTransport::relay(var("EMAIL_SERVER").unwrap().as_str())
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => return true,
        Err(_) => return false,
    }
}
