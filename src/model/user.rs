extern crate serde;
extern crate argonautica;
extern crate regex;
extern crate nanoid;

use serde::{Serialize, Deserialize};
use crate::schema::users;
use argonautica::Hasher;
use argonautica::Verifier;
use regex::Regex;
use std::num::ParseIntError;


///Struct that is utilized to create new users. Post requests made to the /create/user endpoint. Data here will come from the create account form
//Will derive FromForm and Deserialze here
#[derive(FromForm, Serialize)]
pub struct CreateUser
{
    ///User name of the prospective user, this should be unique
    pub(crate) user_name: String,

    ///Email of the prospective user, this should be unique
    pub(crate) email: String,

    ///Password of the prospective user, this is encoded and should not be used anywhere in code. This will get hashed as soon as the create user process is invoked.
    pub(crate) password: String,
}

///Struct representing a representing user logging in. This will be used as user provided data to authenticate and preform authorized actions
//Will derive FromForm and Deserialze here
#[derive(FromForm)]
pub struct LoginUser
{
    ///The username of the user who is attempting to login
    pub(crate) user_name: String,

    ///The non-encrypted password of the user who is trying to login in
    pub(crate) password: String
}

///Struct representing existing user. This should be used strictly to authenticate a user.
///

pub struct AuthInfo
{
    user_id: u32,
    password_hash: String,
}

//For inserting new users into the database
#[table_name = "users"]
#[derive(Insertable)]
pub struct DBUser
{
    #[column_name = "userId"]
    pub user_id: u32,

    #[column_name = "userName"]
    pub user_name: String,

    #[column_name = "passwordHash"]
    pub password_hash: String,

    #[column_name = "email"]
    pub email: String,
}

pub(crate) trait PasswordHash
{
    fn hash_password(&self) -> String;
}

impl CreateUser
{
    //Valid usernames have 3 alphanumeric characters and are not in the list of banned usernames.
    pub(crate) fn is_valid_username(&self) -> bool
    {
        if (self.user_name.chars().all(char::is_alphanumeric)) && (self.user_name.len() >= 3)
        {
            //CHECK FOR BANNED USERNAMES HERE

            true
        }
        else
        {
            false
        }
    }

    pub(crate) fn is_valid_email(&self) -> bool
    {
        //THIS REGEX WILL VALIDATE EMAIL ADDRESSES DO NOT CHANGE
        let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();

        if email_regex.is_match(self.email.as_str())
        {
            true
        }
        else
        {
            false
        }
    }

    pub(crate) fn is_valid_password(&self) -> bool
    {
        if self.password.len() >= 8 && self.has_symbol() && self.has_number() && self.has_capital_letter()
        {
            true
        }
        else
        {
            false
        }
    }

    fn has_number(&self) -> bool
    {
        for character in self.password.chars()
        {
            if character.is_numeric()
            {
                return true
            }
        }
        false
    }

    fn has_symbol(&self)-> bool
    {
        for character in self.password.chars()
        {
            if !(character.is_alphanumeric())
            {
                return true
            }
        }
        false
    }

    fn has_capital_letter(&self) -> bool
    {
        for character in self.password.chars()
        {
            if character.is_uppercase()
            {
                return true
            }
        }
        false
    }


}

impl PasswordHash for CreateUser
{
    fn hash_password(&self) -> String
    {
        let mut hasher = Hasher::default();
        let hash = hasher
            .with_password(self.password.to_string())
            .with_secret_key("cQfTjWnZr4u7x!A%D*F-JaNdRgUkXp2s5v8y/B?E(H+KbPeShVmYq3t6w9z$C&F)J@NcQfTjWnZr4u7x!A%D*G-KaPdSgVkXp2s5v8y/B?E(H+MbQeThWmZq3t6w9z$C&F)J@NcRfUjXn2r5u7x!A%D*G-KaPdSgVkYp3s6v9y/B?E(H+MbQeThWmZq4t7w!z%C&F)J@NcRfUjXn2r5u8x/A?D(G-KaPdSgVkYp3s6v9y$B&E)H@MbQeThWmZq4t")
            .hash()
            .unwrap();

        hash
    }
}

impl DBUser
{
    pub(crate) fn new() -> DBUser
    {
        DBUser
        {
            user_id: 0,
            user_name: "".to_string(),
            password_hash: "".to_string(),
            email: "".to_string(),
        }
    }
}

impl LoginUser
{
    pub(crate) fn verify_password_hash(&self, hash: &String) -> Result<bool, argonautica::Error>
    {

        //TO VERIFY PASSWORDS
        let mut verifier = Verifier::default();
        let is_valid = verifier
            .with_hash(hash)
            .with_password(&self.password)
            .with_secret_key("cQfTjWnZr4u7x!A%D*F-JaNdRgUkXp2s5v8y/B?E(H+KbPeShVmYq3t6w9z$C&F)J@NcQfTjWnZr4u7x!A%D*G-KaPdSgVkXp2s5v8y/B?E(H+MbQeThWmZq3t6w9z$C&F)J@NcRfUjXn2r5u7x!A%D*G-KaPdSgVkYp3s6v9y/B?E(H+MbQeThWmZq4t7w!z%C&F)J@NcRfUjXn2r5u8x/A?D(G-KaPdSgVkYp3s6v9y$B&E)H@MbQeThWmZq4t")
            .verify();

        is_valid
    }
}

pub(crate) fn generate_user_id() -> Result<u32, ParseIntError>
{
    let range: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    nanoid::nanoid!(9, &range).parse::<u32>()
}

impl PasswordHash for LoginUser
{
    fn hash_password(&self) -> String
    {
        let mut hasher = Hasher::default();
        let hash = hasher
            .with_password(self.password.to_string())
            .with_secret_key("cQfTjWnZr4u7x!A%D*F-JaNdRgUkXp2s5v8y/B?E(H+KbPeShVmYq3t6w9z$C&F)J@NcQfTjWnZr4u7x!A%D*G-KaPdSgVkXp2s5v8y/B?E(H+MbQeThWmZq3t6w9z$C&F)J@NcRfUjXn2r5u7x!A%D*G-KaPdSgVkYp3s6v9y/B?E(H+MbQeThWmZq4t7w!z%C&F)J@NcRfUjXn2r5u8x/A?D(G-KaPdSgVkYp3s6v9y$B&E)H@MbQeThWmZq4t")
            .hash()
            .unwrap();

        hash
    }
}