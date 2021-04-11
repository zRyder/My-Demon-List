pub mod tests
{
    #[test]
    pub async fn pass()
    {
        let test = crate::model::user::CreateUser
        {
            user_name: "Test".to_string(),
            email: "".to_string(),
            password: "Kingdom703!".to_string(),
            geometry_dash_username: "".to_string(),
            geometry_dash_password: "".to_string()
        };

        test.hash_password();
    }
}