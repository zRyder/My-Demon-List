extern crate chrono;

use std::net::SocketAddr;

use chrono::{
    Duration,
    prelude,
    Utc,
};
use diesel::{ExpressionMethods, prelude::*, QueryDsl, RunQueryDsl};
use rocket::http::{Cookie, Status, RawStr};
use rocket::http::Cookies;
use rocket::request::Form;
use rocket_contrib::json::Json;

use crate::model::{
    api_response,
    api_response::ApiResponse,
    users::{
        auth,
        session,
        user,
        user::PasswordHash,
        mail,
    },

};

#[post("/create", format="form", data="<create_info>")]
pub fn create_user(db_conn: crate::DbConnection, create_info: Form<user::CreateUser>, ) -> ApiResponse {
    info!("initiating create_user on incoming request");

    use crate::{schema, model};
    use crate::schema::users::dsl::{
        users,
        userId,
        userName,
    };

    /*
    1) Check is Username is valid DONE
    2) Check is Username is available within the database DONE
    3) Check that Password contains number, Symbol, and 8 characters DONE
    4) Hash password DONE
    5) ensure that the email address is valid DONE
    8) Generate UserID DONE
    8) Send verification email maybe? LATER???
    */

    let mut db_user_entry = user::DBUser::default();
    let mut auth_info_entry = auth::AuthInfo::default();

    if user::is_valid_username(&create_info.user_name) {
        info!("username {:?} ok", &create_info.user_name);
        db_user_entry.user_name = create_info.user_name.clone();
    }
    else {
        error!("invalid username {:?}", &create_info.user_name);
        return ApiResponse {
            json: Json("{\"message\": \"invalid username\"}".to_string()),
            status: Status::BadRequest,
        }
    }

    if user::is_valid_password(&create_info.password) {
        info!("password ok");
        auth_info_entry.password_hash = user::hash_password(&create_info.password);;
    }
    else {
        error!("malformed password");
        return ApiResponse {
            json: Json("{\"message\": \"malformed password\"}".to_string()),
            status: Status::BadRequest,
        }
    }

    if user::is_valid_email(&create_info.email) {
        info!("email_address {:?} ok", &create_info.email);
        db_user_entry.email = create_info.email.clone();
    }
    else {
        error!("invalid email_address {:?}", &create_info.email);
        return ApiResponse {
            json: Json("{\"message\": \"invalid email address\"}".to_string()),
            status: Status::BadRequest,
        }
    }

    let user_id =super::super::generate_numeric_id(9);
    info!("generating user_id value = {:?}", &user_id);
        match users.select(userId)
            .filter(userId.eq(&user_id))
            .load::<u32>(&db_conn.0) {
            Ok(user_id_result) => {
                if user_id_result.len() == 0 {
                    info!("user id {:?} is free", &user_id);
                    db_user_entry.user_id = user_id;
                    auth_info_entry.user_id = user_id;
                }
            }
            Err(e) => {
                return api_response::database_error(e)
            }
        }

    return match diesel::insert_into(schema::users::table)
        .values(&db_user_entry)
        .execute(&db_conn.0) {
        Ok(_insert_user_check) => {
            match diesel::insert_into(schema::user_hash::table)
                .values(auth_info_entry)
                .execute(&db_conn.0) {
                Ok(_insert_auth_flag) => {
                    info!("User successfully created with info {:?}", &db_user_entry);
                    info!("initiating new account verification setup");
                    mail::send_verification_email(&db_conn, &db_user_entry.user_id, &db_user_entry.email);
                    ApiResponse {
                        json: Json("{\"message\": \"user added successfully\"}".to_string()),
                        status: Status::Created,
                    }
                }
                Err(e) => {
                    error!("unable to insert new user_hash record, removing from users table");
                    diesel::delete(schema::users::table
                        .filter(userName.eq(&create_info.user_name)))
                        .execute(&db_conn.0);
                    api_response::database_error(e)
                }
            }
        }
        Err(e) => {
            api_response::database_error(e)
        }
    }
}

#[post("/login", format="form", data="<login_info>")]
pub fn login_user(db_conn: crate::DbConnection, login_info: Form<auth::LoginUser>, mut cookies: Cookies) -> ApiResponse {
    use crate::schema::{
        users::dsl::{
            users,
            userName,
            userId
        },
        user_hash::dsl::{
            user_hash,
            passwordHash,
            userId as auth_userId
        },
        user_sessions::dsl::{
            user_sessions
        }
    };
    use crate::model;
    /*
        1) Check if a user with the given username exists
        2) Grab the password hash from the DB and call an authenticate function
        3) If valid, create a private cookie containing the user_id and if the user is an admin (Viprin, RobTop, or Ryder) add an admin cookie as well
    */

    if login_info.user_name.is_empty() || login_info.password.is_empty() {
        return ApiResponse {
            json: Json("{\"message\": \"username or password is empty\"}".to_string()),
            status: Status::BadRequest
        }
    }

    return match users.select(userId).filter(userName.eq(&login_info.user_name)).load::<u32>(&*db_conn) {
        Ok(user_result) => {
            match user_result.first() {
                Some(userid) => {
                    match auth::verify_password_hash(&user_hash.select(passwordHash).filter(auth_userId.eq(userid))
                        .load::<String>(&*db_conn).unwrap().first().unwrap(), &login_info.password) {
                        Ok(is_authenticated) => {
                            if is_authenticated {
                                let session_id = super::super::generate_id(128);
                                let user_cookie: Cookie;
                                let session_info = session::SessionInfo {
                                    user_id: *userid,
                                    session_id: session_id.clone(),
                                    expire_date: (Utc::now() + Duration::days(1)).naive_utc(),
                                };

                                match diesel::replace_into(user_sessions).values(session_info).execute(&db_conn.0) {
                                    Ok(_insert_check) => {
                                        user_cookie = Cookie::build("session", session_id)
                                            .path("/")
                                            .secure(true)
                                            .http_only(true)
                                            .finish();
                                        cookies.add_private(user_cookie);

                                        ApiResponse {
                                            json: Json("{\"message\": \"login successful\"}".to_string()),
                                            status: Status::Ok
                                        }
                                    }
                                    Err(e) => {
                                        api_response::database_error(e)
                                    }
                                }
                            }
                            else {
                                ApiResponse {
                                    json: Json("{\"message\": \"invalid password\"}".to_string()),
                                    status: Status::Forbidden
                                }
                            }
                        }
                        Err(e) => {
                            ApiResponse {
                                json: Json(e.to_string()),
                                status: Status::InternalServerError
                            }
                        }
                    }
                }
                None => {
                    ApiResponse {
                        json: Json("{\"message\": \"user does not exists\"}".to_string()),
                        status: Status::NotFound
                    }
                }
            }
        }
        Err(e) => {
            api_response::database_error(e)
        }
    }
}

#[patch("/update/username", format="form", data="<new_username>")]
pub fn update_username(db_conn: crate::DbConnection, new_username: Form<user::UpdateUserName>, mut cookies: Cookies) -> ApiResponse {
    use crate::schema::{
        users::dsl::{
            users,
            userId,
            userName
        },
    };

    match cookies.get_private("session") {
        Some(session_id_cookie) => {
            match auth::is_valid_session(session_id_cookie.value(), &db_conn) {
                Some(user_id) => {
                    match diesel::update(users.filter(userId.eq(&user_id)))
                        .set(userName.eq(&new_username.user_name))
                        .execute(&db_conn.0) {
                        Ok(_result) => {
                            ApiResponse {
                                json: Json("{\"message\": \"username successfully updated\"}".to_string()),
                                status: Status::Ok
                            }
                        }
                        Err(e) => {
                            api_response::database_error(e)
                        }
                    }
                }
                None => {
                    ApiResponse {
                        json: Json("{\"message\": \"user is not logged in\"}".to_string()),
                        status: Status::Unauthorized
                    }
                }
            }
        }
        None => {
            ApiResponse{
                json: Json("{\"message\": \"user is not logged in\"}".to_string()),
                status: Status::Unauthorized
            }
        }
    }
}

#[patch("/update/password", format="form", data="<new_password>")]
pub fn update_password(db_conn: crate::DbConnection, new_password: Form<user::UpdatePassword>, mut cookies: Cookies) -> ApiResponse {
    use crate::schema::{
        user_hash::dsl::{
            user_hash,
            passwordHash,
            userId
        },
    };

    if !user::is_valid_password(&new_password.new_password)
    {
        return ApiResponse {
            json: Json("{message: new password is invalid}".to_string()),
            status: Status::BadRequest
        }
    }

    match cookies.get_private("session") {
        Some(session_id_cookie) => {
            match auth::is_valid_session(session_id_cookie.value(), &db_conn) {
                Some(user_id) => {
                    match auth::verify_password_hash(&user_hash.select(passwordHash)
                        .filter(userId.eq(user_id))
                        .load::<String>(&*db_conn).unwrap().first().unwrap(),&new_password.current_password) {
                        Ok(is_authenticated) => {
                            if is_authenticated {
                                match diesel::update(user_hash.filter(userId.eq(&user_id)))
                                    .set(passwordHash.eq(&user::hash_password(&new_password.new_password)))
                                    .execute(&db_conn.0) {
                                    Ok(_result) => {
                                        ApiResponse {
                                            json: Json("{\"message\": \"password updated successfully\"}".to_string()),
                                            status: Status::Ok
                                        }
                                    }
                                    Err(e) => {
                                        api_response::database_error(e)
                                    }
                                }
                            }
                            else {
                                ApiResponse {
                                    json: Json("{\"message\": \"current password is incorrect\"}".to_string()),
                                    status: Status::Forbidden
                                }
                            }
                        }
                        Err(e) => {
                            ApiResponse {
                                json: Json(e.to_string()),
                                status: Status::InternalServerError
                            }
                        }
                    }
                }
                None => {
                    ApiResponse {
                        json: Json("{\"message\": \"user is not logged in\"}".to_string()),
                        status: Status::Unauthorized
                    }
                }
            }
        }
        None => {
            ApiResponse{
                json: Json("{message: user is not logged in}".to_string()),
                status: Status::Unauthorized
            }
        }
    }
}

#[post("/verify_account?<verification_id>")]
pub fn verify_user(db_conn: crate::DbConnection, verification_id: &RawStr) -> ApiResponse {
    use crate::schema::{
        user_verification::dsl::{
            user_verification,
            userId,
            expire,
            verificationCode,
        },

        users::dsl::{
            users,
            userId as users_userId,
            isVerified

        }
    };

    info!("initiating user verification {}", verification_id.as_str());
    match user_verification.select((userId, verificationCode, expire))
        .filter(verificationCode.eq(verification_id.as_str()))
        .load::<(u32, String, chrono::NaiveDateTime)>(&*db_conn.0) {
        Ok(user_to_verify) => {
            match user_to_verify.first() {
                Some(verify_user) => {
                    info!("found user with info {:?}", verify_user);

                    if Utc::now().naive_utc() > verify_user.2 {
                        warn!("verification code {:?} has expired for user {:?}", &verification_id.as_str(), &verify_user);
                        return ApiResponse {
                            json: Json("{\"message\": \"verification code has expired\"}".to_string()),
                            status: Status::Conflict
                        }
                    }
                    match diesel::update(users.filter(users_userId.eq(verify_user.0)))
                        .set(isVerified.eq(true))
                        .execute(&db_conn.0) {
                        Ok(_update_check) => {
                            info!("user with info {:?} verified", verify_user);
                            ApiResponse{
                                json: Json("{\"message\": \"user successfully verified\"}".to_string()),
                                status: Status::Ok
                            }
                        }
                        Err(e) => {
                            api_response::database_error(e)
                        }
                    }
                }
                None => {
                    warn!("verification code could not be found");
                    ApiResponse{
                        json: Json("{\"message\": \"no user with given verification code\"}".to_string()),
                        status: Status::NotFound
                    }
                }
            }
        }
        Err(e) => {
            api_response::database_error(e)
        }
    }
}
