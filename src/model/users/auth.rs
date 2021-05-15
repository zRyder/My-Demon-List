use argonautica::{Hasher, Verifier};
use crate::schema::user_hash;

///Struct representing existing user. This should be used strictly to authenticate a user.

#[table_name = "user_hash"]
#[derive(Insertable)]
pub struct AuthInfo
{
    #[column_name = "userId"]
    pub(super) user_id: u32,

    #[column_name = "passwordHash"]
    pub(super) password_hash: String,
}

///Struct representing a representing user logging in. This will be used as user provided data to authenticate and preform authorized actions
//Will derive FromForm
#[derive(FromForm)]
pub struct LoginUser
{
    ///The username of the user who is attempting to login
    pub(super) user_name: String,

    ///The non-encrypted password of the user who is trying to login in
    pub(super) password: String
}

impl Default for AuthInfo
{
    fn default() -> Self
    {
        AuthInfo
        {
            user_id: 0,
            password_hash: "".to_string(),
        }
    }
}

impl LoginUser
{
    pub(super) fn verify_password_hash(&self, hash: &String) -> Result<bool, argonautica::Error>
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

