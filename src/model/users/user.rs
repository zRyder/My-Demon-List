extern crate serde;
extern crate argonautica;
extern crate regex;
extern crate nanoid;

use std::env;
use std::num::ParseIntError;

use argonautica::Hasher;
use dotenv;
use regex::Regex;
use serde::Serialize;

use crate::schema::users;

///Struct that is utilized to create new users. Post requests made to the /create/user endpoint. Data here will come from the create account form
#[derive(FromForm)]
pub struct CreateUser {
    ///User name of the prospective user, this should be unique
    pub(crate) user_name: String,

    ///Email of the prospective user, this should be unique
    pub(crate) email: String,

    ///Password of the prospective user, this is encoded and should not be used anywhere in code. This will get hashed as soon as the create user process is invoked.
    pub(crate) password: String,
}

//For inserting new users into the database
#[table_name = "users"]
#[derive(Insertable)]
pub struct DBUser {
    #[column_name = "userId"]
    pub user_id: u32,

    #[column_name = "userName"]
    pub user_name: String,

    #[column_name = "email"]
    pub email: String,
}

#[derive(FromForm)]
pub struct UpdateUserName {
    pub(crate) _method: String,
    pub(crate) user_name: String
}

#[derive(FromForm)]
pub struct UpdatePassword {
    pub(crate) _method: String,
    pub(crate) current_password: String,
    pub(crate) new_password: String
}

pub(crate) trait PasswordHash {
    fn hash_password(&self) -> String;
}

pub(super) fn is_valid_username(user_name: &String) -> bool {
    if (user_name.chars().all(char::is_alphanumeric)) && (user_name.len() >= 3) {
        //CHECK FOR BANNED USERNAMES HERE

        true
    }
    else {
        false
    }
}

pub(super) fn is_valid_email(email: &String) -> bool {
    //THIS REGEX WILL VALIDATE EMAIL ADDRESSES DO NOT CHANGE
    let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();

    if email_regex.is_match(email) {
        true
    }
    else {
        false
    }
}

pub(super) fn is_valid_password(password: &String) -> bool {
    if password.len() >= 8 && has_symbol(password) && has_number(password) && has_capital_letter(password) {
        true
    }
    else {
        false
    }
}

pub(super) fn has_number(password: &String) -> bool {
    for character in password.chars() {
        if character.is_numeric() {
            return true
        }
    }
    false
}

pub(super) fn has_symbol(password: &String)-> bool {
    for character in password.chars() {
        if !(character.is_alphanumeric()) {
            return true
        }
    }
    false
}

pub(super) fn has_capital_letter(password: &String) -> bool {
    for character in password.chars() {
        if character.is_uppercase() {
            return true
        }
    }
    false
}

pub(super) fn hash_password(password: &String) -> String
{
    dotenv::dotenv().ok();
    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(password)
        .with_secret_key(&std::env::var("SECRET_HASH").unwrap())
        .hash()
        .unwrap();

    hash
}

impl PasswordHash for CreateUser
{
    fn hash_password(&self) -> String
    {
        dotenv::dotenv().ok();
        let mut hasher = Hasher::default();
        let hash = hasher
            .with_password(self.password.to_string())
            .with_secret_key(&std::env::var("SECRET_HASH").unwrap())
            .hash()
            .unwrap();

        hash
    }
}

impl Default for DBUser
{
    fn default() -> Self
    {
        DBUser
        {
            user_id: 0,
            user_name: "".to_string(),
            email: "".to_string()
        }
    }
}