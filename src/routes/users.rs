use crate::misc::appstate::AppState;
use crate::misc::hasher::{hash, verify};
use crate::misc::jwt::{generate_token, get_id_from_request};
use crate::misc::utils::get_id;
use crate::misc::utils::is_available_username;
use crate::misc::validator::{get_valid_username, validate_email};
use crate::models::posts::PostOut;
use crate::models::users::{
    CreateUser, EmailOTP, IdPassword, JWTResponse, LoginUser, UpdateUser, UserOut, UserOutPublic,
};
use crate::models::Response;
use crate::routes::helper::{create_otp_and_send_email, forgot_password_email, get_updated_user};
use actix_web::{delete, get, patch, post, web, HttpRequest, HttpResponse};

#[post("/users")]
pub async fn create_user(
    app_state: web::Data<AppState>,
    form: web::Json<CreateUser>,
) -> HttpResponse {
    if !validate_email(&form.email) {
        return HttpResponse::BadRequest().json(Response {
            message: "Enter a valid email address".to_string(),
        });
    }
    let username_result = get_valid_username(&form.username);
    if username_result.is_none() {
        return HttpResponse::BadRequest().json(Response {
            message: "Username invalid.".to_string(),
        });
    }
    let username = username_result.unwrap();
    if is_available_username(&username, &app_state).await == false {
        return HttpResponse::BadRequest().json(Response {
            message: "Username already taken".to_string(),
        });
    }
    let hashed_pw = hash(&form.password);
    let created = sqlx::query_as!(UserOut,
        "INSERT INTO users (email, username, password) values ($1, $2, $3) RETURNING id, email, username, phone, first_name, last_name, bio, address, profile_pic, credits, email_verified, phone_verified, email_public, phone_public, is_active, created_at, updated_at",
        &form.email,
        username,
        &hashed_pw
    )
    .fetch_one(&app_state.pool)
    .await;
    if created.is_err() {
        return HttpResponse::BadRequest().json(Response {
            message: "User already exists".to_string(),
        });
    }
    return HttpResponse::Created().json(created.unwrap());
}

#[get("/users/login")]
pub async fn login_user(
    app_state: web::Data<AppState>,
    form: web::Json<LoginUser>,
) -> HttpResponse {
    let error_response = HttpResponse::BadRequest().json(Response {
        message: "Login unsuccessful, wrong email or username or password".to_string(),
    });
    let pw_result;
    if validate_email(&form.credential) {
        pw_result = sqlx::query_as!(
            IdPassword,
            "SELECT id, password FROM users WHERE email=$1",
            &form.credential
        )
        .fetch_one(&app_state.pool)
        .await;
    } else {
        let username = get_valid_username(&form.credential);
        pw_result = sqlx::query_as!(
            IdPassword,
            "SELECT id, password FROM users WHERE username=$1",
            username
        )
        .fetch_one(&app_state.pool)
        .await;
    }

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
pub async fn refresh_token(req: HttpRequest, app_state: web::Data<AppState>) -> HttpResponse {
    let user_id_result = get_id_from_request(&req, &app_state);
    match user_id_result.await {
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

#[get("/users/me")]
pub async fn get_current_user(req: HttpRequest, app_state: web::Data<AppState>) -> HttpResponse {
    let user_id_result = get_id_from_request(&req, &app_state);
    match user_id_result.await {
        Ok(val) => {
            let user_result = sqlx::query_as!(UserOut, "SELECT id, email, username, phone, first_name, last_name, bio, address, profile_pic, credits, email_verified, phone_verified, email_public, phone_public, is_active, created_at, updated_at FROM users WHERE id=$1", val).fetch_one(&app_state.pool).await;
            match user_result {
                Ok(user) => {
                    return HttpResponse::Ok().json(user);
                }
                Err(_) => {
                    return HttpResponse::InternalServerError().json(Response {
                        message: "Something went wrong, try again later".to_string(),
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
pub async fn get_email_otp(req: HttpRequest, app_state: web::Data<AppState>) -> HttpResponse {
    let user_id_result = get_id_from_request(&req, &app_state);
    let user_id: i32;
    match user_id_result.await {
        Ok(id) => {
            user_id = id;
        }
        Err(e) => {
            return HttpResponse::Unauthorized().json(Response {
                message: e.to_string(),
            });
        }
    }

    let query_result = sqlx::query_as!(
        EmailOTP,
        "SELECT id, email, email_verified FROM users WHERE id=$1",
        user_id
    )
    .fetch_one(&app_state.pool)
    .await;
    if query_result.is_err() {
        return HttpResponse::InternalServerError().json(Response {
            message: "Something went wrong, try again later".to_string(),
        });
    }
    let user = query_result.unwrap();

    if user.email_verified {
        return HttpResponse::BadRequest().json(Response {
            message: "User already verified".to_string(),
        });
    }

    let res = create_otp_and_send_email(user.id, user.email, &app_state).await;
    if res {
        return HttpResponse::Ok().json(Response {
            message: "Email sent successfully".to_string(),
        });
    } else {
        return HttpResponse::InternalServerError().json(Response {
            message: "Something went wrong, try again later".to_string(),
        });
    }
}

// If a wrong username is given by the user, this endpoint just ignores that specific input
#[patch("/users/me")]
pub async fn update_user(
    req: HttpRequest,
    form: web::Json<UpdateUser>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let user_id_result = get_id_from_request(&req, &app_state);
    let user_id: i32;
    match user_id_result.await {
        Ok(id) => {
            user_id = id;
        }
        Err(e) => {
            return HttpResponse::Unauthorized().json(Response {
                message: e.to_string(),
            });
        }
    }
    let updated_user = get_updated_user(user_id, &form, &app_state).await;
    let q = sqlx::query_as!(UserOut, r#"
        UPDATE users SET 
        email=$1, username=$2, phone=$3, first_name=$4, last_name=$5, bio=$6, address=$7, email_verified=$8, phone_verified=$9, email_public=$10, phone_public=$11
        WHERE id=$12
        RETURNING id, email, username, phone, first_name, last_name, bio, address, profile_pic, credits, email_verified, phone_verified, email_public, phone_public, is_active, created_at, updated_at
        "#, 
        updated_user.email, updated_user.username, updated_user.phone, updated_user.first_name, updated_user.last_name, updated_user.bio, updated_user.address, updated_user.email_verified, updated_user.phone_verified, updated_user.email_public, updated_user.phone_public, user_id).fetch_one(&app_state.pool).await;
    if q.is_err() {
        HttpResponse::InternalServerError().json(Response {
            message: "Something went wrong, try again later".to_string(),
        });
    }
    return HttpResponse::Ok().json(q.unwrap());
}

#[delete("/users/me")]
pub async fn delete_user(req: HttpRequest, app_state: web::Data<AppState>) -> HttpResponse {
    let user_id_result = get_id_from_request(&req, &app_state);
    let user_id: i32;
    match user_id_result.await {
        Ok(id) => {
            user_id = id;
        }
        Err(e) => {
            return HttpResponse::Unauthorized().json(Response {
                message: e.to_string(),
            });
        }
    }
    let q2 = sqlx::query!("SELECT id from delete_users where user_id=$1", user_id)
        .fetch_one(&app_state.pool)
        .await;
    if q2.is_ok() {
        return HttpResponse::BadRequest().json(Response {
            message: "Your account is already queued for deletion".to_string(),
        });
    }
    let q = sqlx::query!("INSERT INTO delete_users (user_id) values ($1)", user_id)
        .execute(&app_state.pool)
        .await;
    let q3 = sqlx::query!("UPDATE users SET is_active='f' WHERE id=$1", user_id)
        .execute(&app_state.pool)
        .await;
    if q.is_err() || q3.is_err() { // Ideally, the above two queries should be within a single transaction so as to roll back (ACID compliance)
        return HttpResponse::InternalServerError().json(Response {
            message: "Something went wrong, try again later".to_string(),
        });
    }
    return HttpResponse::Ok().json(Response {
        message: "Your account is queued for deletion".to_string(),
    });
}

// Undelete needs a different implementation. This doesn't work.
#[get("/users/undelete")]
pub async fn undelete_user(req: HttpRequest, app_state: web::Data<AppState>) -> HttpResponse {
    let user_id_result = get_id_from_request(&req, &app_state);
    let user_id: i32;
    match user_id_result.await {
        Ok(id) => {
            user_id = id;
        }
        Err(e) => {
            return HttpResponse::Unauthorized().json(Response {
                message: e.to_string(),
            });
        }
    }
    // Check this function if the user can request for undeletion if he is inactive
    let q2 = sqlx::query!("SELECT id FROM delete_users WHERE user_id=$1", user_id)
        .fetch_one(&app_state.pool)
        .await;
    if q2.is_err() {
        return HttpResponse::BadRequest().json(Response {
            message: "Your account is not queued for deletion".to_string(),
        });
    }
    let q = sqlx::query!("DELETE from delete_users where user_id=$1", user_id)
        .execute(&app_state.pool)
        .await;
    if q.is_err() {
        dbg!(&q);
        return HttpResponse::InternalServerError().json(Response {
            message: "Something went wrong, try again later".to_string(),
        });
    }
    return HttpResponse::Ok().json(Response {
        message: "Your account has been removed from the delete queue".to_string(),
    });
}

#[get("/users/{id}")]
pub async fn get_user(path: web::Path<String>, app_state: web::Data<AppState>) -> HttpResponse {
    let user_string = path.into_inner();
    let user_id_opt = get_id(&user_string);
    if user_id_opt.is_some() {
        let user_id = user_id_opt.unwrap();
        let q1 = sqlx::query_as!(UserOut, "SELECT id, email, username, phone, first_name, last_name, bio, address, profile_pic, credits, email_verified, phone_verified, email_public, phone_public, is_active, created_at, updated_at FROM users WHERE id=$1", user_id).fetch_one(&app_state.pool).await;
        if q1.is_err() {
            return HttpResponse::NotFound().json(Response {
                message: format!("User with id: {} not found in the database", user_id),
            });
        }
        let user_out: UserOutPublic = q1.unwrap().get_public_user();
        return HttpResponse::Ok().json(user_out);
    }
    let q2 = sqlx::query_as!(UserOut, "SELECT id, email, username, phone, first_name, last_name, bio, address, profile_pic, credits, email_verified, phone_verified, email_public, phone_public, is_active, created_at, updated_at FROM users WHERE username=$1", user_string).fetch_one(&app_state.pool).await;
    if q2.is_err() {
        return HttpResponse::NotFound().json(Response {
            message: format!(
                "User with username: {} not found in the database",
                user_string
            ),
        });
    }
    let user_out: UserOutPublic = q2.unwrap().get_public_user();
    return HttpResponse::Ok().json(user_out);
}

#[get("/users/{id}/posts")]
pub async fn get_users_posts(
    path: web::Path<String>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let username = path.into_inner();
    let user_id_opt = get_id(&username);
    if user_id_opt.is_some() {
        let user_id = user_id_opt.unwrap();
        let q1 = sqlx::query_as!(PostOut, r#"
            SELECT id, title, user_id, brand, post_pic, price, model_year, km_driven, transmission as "transmission: _", fuel as "fuel: _", description, location, is_sold, created_at, updated_at
            FROM posts WHERE user_id=$1
            "#, user_id).fetch_all(&app_state.pool).await;
        if q1.is_err() {
            return HttpResponse::InternalServerError().json(Response {
                message: "Something went wrong, try again later".to_string(),
            });
        }
        return HttpResponse::Ok().json(q1.unwrap());
    }
    let q2 = sqlx::query_as!(PostOut, r#"
        SELECT id, title, user_id, brand, post_pic, price, model_year, km_driven, transmission as "transmission: _", fuel as "fuel: _", description, location, is_sold, created_at, updated_at
        FROM posts WHERE user_id=(SELECT users.id FROM users WHERE username=$1)
        "#, username).fetch_all(&app_state.pool).await;
    if q2.is_err() {
        return HttpResponse::InternalServerError().json(Response {
            message: "Something went wrong, try again later".to_string(),
        });
    }
    return HttpResponse::Ok().json(q2.unwrap());
}

#[get("/users/forgotpassword/{id}")]
pub async fn forgot_password(path: web::Path<String>, app_state: web::Data<AppState>) -> HttpResponse {
    let id = path.into_inner();
    let id_int = get_id(&id);
    if id_int.is_some() {
        let user_id = id_int.unwrap();
        let user_email_result = sqlx::query!("SELECT email FROM users WHERE id=$1", user_id).fetch_one(&app_state.pool).await;
        if user_email_result.is_err() {
            return HttpResponse::BadRequest().json(Response{
                message: format!("User with id {} is not found in the database", user_id)
            });
        }
        let user_email = user_email_result.unwrap();
        let u_email = user_email.email;
        return forgot_password_email(&u_email, &app_state).await;
    }
    if validate_email(&id) {
        let user_id_result = sqlx::query!("SELECT id FROM users WHERE email=$1", &id).fetch_one(&app_state.pool).await;
        if user_id_result.is_err() {
            return HttpResponse::BadRequest().json(Response{
                message: format!("User with email {} not found in the database", id)
            });
        }
        let user_id = user_id_result.unwrap().id;
        return forgot_password_email(&id, &app_state).await;
    }
    let username_result = sqlx::query!("SELECT id, email FROM users WHERE username=$1", &id).fetch_one(&app_state.pool).await;
    if username_result.is_err() {
        return HttpResponse::BadRequest().json(Response{
            message: format!("User with username {} not found in database", &id)
        });
    }
    let user_res = username_result.unwrap();
    return forgot_password_email(&user_res.email, &app_state).await;
}

#[post("/users/changepassword/{id}")]
pub async fn change_password(path: web::Path<String>, app_state: web::Data<AppState>) -> HttpResponse {
    return HttpResponse::Ok().json("Hi");
}