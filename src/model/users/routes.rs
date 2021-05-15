use diesel::{ExpressionMethods, prelude::*, QueryDsl, RunQueryDsl};
use rocket::http::{Cookie, Status};
use rocket::http::Cookies;
use rocket::request::Form;
use rocket_contrib::json::Json;

use crate::model::
{
    api_response,
    api_response::ApiResponse,
    users::{
        user,
        user::PasswordHash,
        auth
    },

};

#[post("/users/create", format="form", data="<create_info>")]
pub fn create_user(db_conn: crate::DbConnection, create_info: Form<user::CreateUser>) -> ApiResponse
{
    use crate::schema;
    use crate::schema::users::dsl::
    {
        users,
        userId,
        userName,
        email,
    };

    use crate::schema::user_hash::dsl::
    {
        user_hash,
        userId as auth_userId
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

    if create_info.is_valid_username() && (users.select(userName).filter(userName.eq(create_info.user_name.clone())).load::<String>(&*db_conn).unwrap().len() ==0)
    {
        db_user_entry.user_name = create_info.user_name.clone();
    }
    else
    {
        return ApiResponse
        {
            json: Json("{error: invalid username}".to_string()),
            status: Status::BadRequest,
        }
    }

    if create_info.is_valid_password()
    {
        auth_info_entry.password_hash = create_info.hash_password();;
    }
    else
    {
        return ApiResponse
        {
            json: Json("{error: invalid password}".to_string()),
            status: Status::BadRequest,
        }
    }

    if create_info.is_valid_email()
    {
        db_user_entry.email = create_info.email.clone();
    }
    else
    {
        return ApiResponse
        {
            json: Json("{error: invalid email address}".to_string()),
            status: Status::BadRequest,
        }
    }

    loop
    {
        match user::generate_user_id()
        {
            Ok(user_id) =>
            {
                let user_id_check = users.select(userId).filter(userId.eq(&user_id)).load::<u32>(&*db_conn);

                match user_id_check
                {
                    Ok(user_id_result) =>
                    {
                        if user_id_result.len() == 0
                        {
                            db_user_entry.user_id = user_id;
                            auth_info_entry.user_id = user_id;
                            break;
                        }
                    }
                    Err(e) =>
                    {
                        return api_response::database_error(e)
                    }
                }
            }
            Err(e) =>
            {
                return ApiResponse
                {
                    json: Json(e.to_string()),
                    status: Status::InternalServerError,
                }
            }
        }
    }

    return match diesel::insert_into(schema::users::table).values(db_user_entry).execute(&db_conn.0)
    {
        Ok(_insert_user_check) =>
        {
            match diesel::insert_into(schema::user_hash::table).values(auth_info_entry).execute(&db_conn.0)
            {
                Ok(_insert_auth_flag) =>
                {
                    ApiResponse
                    {
                        json: Json("message: user added successfully".to_string()),
                        status: Status::Ok,
                    }
                }
                Err(e) =>
                {
                    diesel::delete(schema::users::table.filter(userName.eq(&create_info.user_name))).execute(&db_conn.0);
                    api_response::database_error(e)
                }
            }
        }
        Err(e) =>
        {
            api_response::database_error(e)
        }
    }
}

#[post("/users/login", format="form", data="<login_info>")]
pub fn login_user(db_conn: crate::DbConnection, login_info: Form<auth::LoginUser>, mut cookies: Cookies) -> ApiResponse
{
    use crate::schema::
    {
        users::dsl::
        {
            users,
            userName,
            userId
        },
        user_hash::dsl::
        {
            user_hash,
            passwordHash,
            userId as auth_userId
        }
    };
    /*
        1) Check if a user with the given username exists
        2) Grab the password hash from the DB and call an authenticate function
        3) If valid, create a private cookie containing the user_id and if the user is an admin (Viprin, RobTop, or Ryder) add an admin cookie as well
    */

    if login_info.user_name.is_empty() || login_info.password.is_empty()
    {
        return ApiResponse
        {
            json: Json("username or password is empty".to_string()),
            status: Status::BadRequest
        }
    }

    let db_user_result = users.select(userId).filter(userName.eq(&login_info.user_name)).load::<u32>(&*db_conn);
    return match db_user_result
    {
        Ok(user_result) =>
            {
                match user_result.first()
                {
                    Some(userid) =>
                        {
                            let user_auth_info = user_hash.select(passwordHash).filter(auth_userId.eq(userid)).load::<String>(&*db_conn).unwrap();
                            match login_info.verify_password_hash(&user_auth_info.first().unwrap())
                            {
                                Ok(is_authenticated) =>
                                    {
                                        if is_authenticated
                                        {
                                            let user_cookie: Cookie;
                                            user_cookie = Cookie::build("user_id", userid.to_string())
                                                .path("/")
                                                .finish();
                                            cookies.add_private(user_cookie);
                                            println!("I ran");

                                            ApiResponse
                                            {
                                                json: Json("login successful".to_string()),
                                                status: Status::Ok
                                            }
                                        }
                                        else
                                        {
                                            ApiResponse
                                            {
                                                json: Json("invalid password".to_string()),
                                                status: Status::Forbidden
                                            }
                                        }
                                    }
                                Err(e) =>
                                    {
                                        ApiResponse
                                        {
                                            json: Json(e.to_string()),
                                            status: Status::InternalServerError
                                        }
                                    }
                            }
                        }
                    None =>
                        {
                            ApiResponse
                            {
                                json: Json("user does not exists".to_string()),
                                status: Status::BadRequest
                            }
                        }
                }
            }
        Err(e) =>
            {
                api_response::database_error(e)
            }

    }
}
