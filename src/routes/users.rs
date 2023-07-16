use crate::misc::appstate::AppState;
use crate::misc::hasher::{hash, verify};
use crate::misc::jwt::{generate_token, get_id_from_request};
use crate::models::users::{CreateUser, IdPassword, JWTResponse, LoginUser, UserOut, EmailOTP};
use crate::models::Response;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use regex::Regex;

#[post("/users")]
pub async fn create_user(
    app_state: web::Data<AppState>,
    form: web::Json<CreateUser>,
) -> HttpResponse {
    let email_regex: Regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    if !email_regex.is_match(&form.email) {
        return HttpResponse::BadRequest().json(Response {
            message: "Enter a valid email address".to_string(),
        });
    }
    let hashed_pw = hash(&form.password);
    let created = sqlx::query!(
        "INSERT INTO users (email, password) values ($1, $2) RETURNING *",
        &form.email,
        &hashed_pw
    )
    .fetch_one(&app_state.pool)
    .await;
    if created.is_err() {
        return HttpResponse::BadRequest().json(Response {
            message: "User already exists".to_string(),
        });
    }
    return HttpResponse::Created().json(Response {
        message: "User created successfully".to_string(),
    });
}

#[get("/users/login")]
pub async fn login_user(
    app_state: web::Data<AppState>,
    form: web::Json<LoginUser>,
) -> HttpResponse {
    let error_response = HttpResponse::BadRequest().json(Response {
        message: "Login unsuccessful, wrong email or password".to_string(),
    });
    let pw_result = sqlx::query_as!(
        IdPassword,
        "SELECT id, password FROM users WHERE email=$1",
        &form.email
    )
    .fetch_one(&app_state.pool)
    .await;

    if pw_result.is_err() {
        // Wrong password check takes time as it verifies the hash using argon2
        // But, returning from here is much faster since there is no hashing.
        // So, someone can find out if an email is registered or not by calculating the delay
        // So, hash the password once for slowing the return.
        let _ = hash(&form.password);
        return error_response;
    }

    let id_password = pw_result.unwrap();

    let is_valid = verify(&form.password, &id_password.password);

    if is_valid {
        let token = generate_token(id_password.id).unwrap();
        let response = JWTResponse { jwt: token };
        return HttpResponse::Ok().json(response);
    } else {
        return error_response;
    }
}

#[get("/users/refreshtoken")]
pub async fn refresh_token(req: HttpRequest) -> HttpResponse {
    let user = get_id_from_request(&req);
    match user {
        Ok(val) => {
            let token = generate_token(val).unwrap();
            let response = JWTResponse { jwt: token };
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            return HttpResponse::Unauthorized().json(Response {
                message: e.to_string(),
            });
        }
    }
}

#[get("/users/current_user")]
pub async fn get_current_user(req: HttpRequest, app_state: web::Data<AppState>) -> HttpResponse {
    let user = get_id_from_request(&req);
    match user {
        Ok(val) => {
            let user_result = sqlx::query_as!(UserOut, "SELECT email, username, phone, bio, address, profile_pic_url, credits, email_verified, phone_verified, is_active, created_at, updated_at FROM users WHERE id=$1", val).fetch_one(&app_state.pool).await;
            match user_result {
                Ok(user) => {
                    return HttpResponse::Ok().json(user);
                }
                Err(e) => {
                    return HttpResponse::BadRequest().json(Response {
                        message: "It's a valid bearer token, but the user could not be found in the database.".to_string(),
                    });
                }
            }
        }
        Err(e) => {
            return HttpResponse::Unauthorized().json(Response {
                message: e.to_string(),
            });
        }
    }
}

#[get("/users/emailotp")]
pub async fn get_email_otp (req: HttpRequest, app_state: web::Data<AppState>) -> HttpResponse {
    let user_id = get_id_from_request(&req);
    match user_id {
        Ok(id) => {
            let user_result = sqlx::query_as!(EmailOTP, "SELECT email, email_verified, is_active, FROM users WHERE id=$1", val).fetch_one(&app_state.pool).await;
            match user_result {
                Ok(user) => {
                    if user.email_verified {
                        return HttpResponse::BadRequest().json(Response{message: "Email already verified"});
                    } else {
                        send_email();
                    }
                }
                Err(e) => {
                    return HttpResponse::BadRequest().json(Response{message: "User not found in the database".to_string();})
                }
            }
        }
        Err(e) => {
            return HttpResponse::Unauthorized().json(Response {
                message: e.to_string(),
            });
        }
    }
}