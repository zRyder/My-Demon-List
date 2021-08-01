pub mod tests
{
    use crate::model;
    use crate::model::users::{
        email,
        user,
        user::PasswordHash};

    #[test]
    pub async fn pass() {
        let test = crate::model::users::user::CreateUser {
            user_name: "Test".to_string(),
            email: "".to_string(),
            password: "Kingdom703!".to_string(),
        };

        println!("{}", test.hash_password());
    }

    #[test]
    pub async fn session_id_test() {
        let id = super::super::super::generate_id(128);

        println!("{}", id);
    }

    #[test]
    pub async fn valid_password_test() {
        let test = crate::model::users::user::CreateUser {
            user_name: "Test".to_string(),
            email: "".to_string(),
            password: "Kingdom703@".to_string(),
        };
        assert_eq!(true, user::is_valid_password(&test.password));

        let test2 = crate::model::users::user::CreateUser {
            user_name: "Test".to_string(),
            email: "".to_string(),
            password: "Ki!3".to_string(),
        };
        assert_eq!(false, user::is_valid_password(&test2.password));

        let test3 = crate::model::users::user::CreateUser {
            user_name: "Test".to_string(),
            email: "".to_string(),
            password: "Kingdom703".to_string(),
        };
        assert_eq!(false, user::is_valid_password(&test3.password));

        let test4 = crate::model::users::user::CreateUser {
            user_name: "Test".to_string(),
            email: "".to_string(),
            password: "Kingdom@".to_string(),
        };
        assert_eq!(false, user::is_valid_password(&test4.password));
    }

    #[test]
    pub async fn valid_email_test()
    {
        let test = crate::model::users::user::CreateUser {
            user_name: "Test".to_string(),
            email: "something@domain.com".to_string(),
            password: "".to_string(),
        };
        assert_eq!(true, user::is_valid_email(&test.email));

        let test2 = crate::model::users::user::CreateUser {
            user_name: "Test".to_string(),
            email: "somethingatdomain.com".to_string(),
            password: "".to_string(),
        };
        assert_eq!(false, user::is_valid_email(&test2.email));

        let test3 = crate::model::users::user::CreateUser {
            user_name: "Test".to_string(),
            email: "@domain.com".to_string(),
            password: "".to_string(),
        };
        assert_eq!(false, user::is_valid_email(&test3.email));

        let test4 = user::CreateUser {
            user_name: "Test".to_string(),
            email: "something@domain.co.uk".to_string(),
            password: "".to_string(),
        };
        assert_eq!(true, user::is_valid_email(&test4.email));
    }

    #[test]
    pub async fn generate_user_id_test() {
        let mut user_id = super::super::super::generate_numeric_id(9) ;
        println!("{}", user_id);

        user_id = super::super::super::generate_numeric_id(9);
        println!("{}", user_id);
    }

    #[test]
    pub async fn test_email() {
        // email::send_verification_email(crate::DbConnection::, 421280988);
    }
}