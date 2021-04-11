extern crate serde;
extern crate argonautica;


use serde::{Serialize, Deserialize};
use crate::schema::users;


use argonautica::Hasher;
use argonautica::Verifier;

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

    ///The Geometry Dash username of prospective user, this will not be stored in the database table and will serve as a method to get the Geometry Dash User ID which will be stored in My Demon List's users database
    pub(crate) geometry_dash_username: String,

    ///The Geometry Dash password of prospective user, this will not be stored in the database table and will serve as a method to get the Geometry Dash User ID which will be stored in My Demon List's users database
    pub(crate) geometry_dash_password: String,
}

///Struct representing a representing user logging in. This will be used as user provided data to authenticate and preform authorized actions
//Will derive FromForm and Deserialze here
#[derive(FromForm)]
pub struct LoginUser
{
    ///The username of the user who is attempting to login
    user_name: String,

    ///The non-encrypted password of the user who is trying to login in
    password: String
}

///Struct representing existing user. This should be used strictly to authenticate a user.
//#[derive(Insertable)]
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
    pub user_id: i32,

    #[column_name = "userName"]
    pub user_name: String,

    #[column_name = "passwordHash"]
    pub password_hash: String,

    #[column_name = "email"]
    pub email: String,

    #[column_name = "gdUserId"]
    pub gd_user_id: String,
}

impl CreateUser
{
    //Valid usernames have 3 alphanumeric characters and are not in the list of banned usernames
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

    pub(crate) fn password_has_number(&self) -> bool
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

    pub(crate) fn password_has_symbol(&self)-> bool
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

    pub(crate) fn hash_password(&self)
    {
        let mut hasher = Hasher::default();
        let hash = hasher
            .with_password(self.password.to_string())
            .with_secret_key("cQfTjWnZr4u7x!A%D*F-JaNdRgUkXp2s5v8y/B?E(H+KbPeShVmYq3t6w9z$C&F)J@NcQfTjWnZr4u7x!A%D*G-KaPdSgVkXp2s5v8y/B?E(H+MbQeThWmZq3t6w9z$C&F)J@NcRfUjXn2r5u7x!A%D*G-KaPdSgVkYp3s6v9y/B?E(H+MbQeThWmZq4t7w!z%C&F)J@NcRfUjXn2r5u8x/A?D(G-KaPdSgVkYp3s6v9y$B&E)H@MbQeThWmZq4t")
            .hash()
            .unwrap();

        println!("{}", &hash);

        //TO VERIFY PASSWORDS
        let mut verifier = Verifier::default();
        let is_valid = verifier
            .with_hash(hash.clone())
            .with_password("Kingdom703@")
            .with_secret_key("cQfTjWnZr4u7x!A%D*F-JaNdRgUkXp2s5v8y/B?E(H+KbPeShVmYq3t6w9z$C&F)J@NcQfTjWnZr4u7x!A%D*G-KaPdSgVkXp2s5v8y/B?E(H+MbQeThWmZq3t6w9z$C&F)J@NcRfUjXn2r5u7x!A%D*G-KaPdSgVkYp3s6v9y/B?E(H+MbQeThWmZq4t7w!z%C&F)J@NcRfUjXn2r5u8x/A?D(G-KaPdSgVkYp3s6v9y$B&E)H@MbQeThWmZq4t")
            .verify()
            .unwrap();

        println!("{}", is_valid);
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
            gd_user_id: "".to_string()
        }
    }
}