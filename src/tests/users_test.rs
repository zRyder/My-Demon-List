pub mod tests
{
    use std::error::Error;
    use crate::model::user::PasswordHash;

    #[test]
    pub async fn pass()
    {
        let test = crate::model::user::CreateUser
        {
            user_name: "Test".to_string(),
            email: "".to_string(),
            password: "Kingdom703!".to_string(),
        };

        println!("{}", test.hash_password());
    }

    #[test]
    pub async fn valid_password_test()
    {
        let test = crate::model::user::CreateUser
        {
            user_name: "Test".to_string(),
            email: "".to_string(),
            password: "Kingdom703@".to_string(),
        };
        assert_eq!(true, test.is_valid_password());

        let test2 = crate::model::user::CreateUser
        {
            user_name: "Test".to_string(),
            email: "".to_string(),
            password: "Ki!3".to_string(),
        };
        assert_eq!(false, test2.is_valid_password());

        let test3 = crate::model::user::CreateUser
        {
            user_name: "Test".to_string(),
            email: "".to_string(),
            password: "Kingdom703".to_string(),
        };
        assert_eq!(false, test3.is_valid_password());

        let test4 = crate::model::user::CreateUser
        {
            user_name: "Test".to_string(),
            email: "".to_string(),
            password: "Kingdom@".to_string(),
        };
        assert_eq!(false, test4.is_valid_password());
    }

    #[test]
    pub async fn valid_email_test()
    {
        let test = crate::model::user::CreateUser
        {
            user_name: "Test".to_string(),
            email: "something@domain.com".to_string(),
            password: "".to_string(),
        };
        assert_eq!(true, test.is_valid_email());

        let test2 = crate::model::user::CreateUser
        {
            user_name: "Test".to_string(),
            email: "somethingatdomain.com".to_string(),
            password: "".to_string(),
        };
        assert_eq!(false, test2.is_valid_email());

        let test3 = crate::model::user::CreateUser
        {
            user_name: "Test".to_string(),
            email: "@domain.com".to_string(),
            password: "".to_string(),
        };
        assert_eq!(false, test3.is_valid_email());

        let test4 = crate::model::user::CreateUser
        {
            user_name: "Test".to_string(),
            email: "something@domain.co.uk".to_string(),
            password: "".to_string(),
        };
        assert_eq!(true, test4.is_valid_email());
    }

    #[test]
    pub async fn generate_user_id_test()
    {
        match crate::model::user::generate_user_id()
        {
            Ok(user_id) =>
                {
                    println!("{}", user_id)
                },
            Err(e) =>
                {
                    println!("{}", e.to_string())
                }
        }

        match crate::model::user::generate_user_id()
        {
            Ok(user_id) =>
                {
                    println!("{}", user_id)
                },
            Err(e) =>
                {
                    println!("{}", e.to_string())
                }
        }
    }
}