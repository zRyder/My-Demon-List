extern crate lettre;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use chrono::{Utc, Duration};
use diesel::{ExpressionMethods, prelude::*, QueryDsl, RunQueryDsl};
use crate::schema::user_verification;
use self::lettre::transport::smtp::response::Response;
use self::lettre::transport::smtp::Error;

#[table_name = "user_verification"]
#[derive(Insertable)]
pub struct VerificationInfo {
    #[column_name = "userId"]
    pub(super) user_id: u32,

    #[column_name = "verificationCode"]
    pub(super) verification_code: String,

    #[column_name = "expire"]
    pub(super) expire_date: chrono::NaiveDateTime
}

impl VerificationInfo {
    pub fn new(user_id: &u32, verification_code: &str, expire_date: &chrono::NaiveDateTime) -> Self {
        VerificationInfo {
            user_id: *user_id,
            verification_code: (*verification_code).parse().unwrap(),
            expire_date: *expire_date,
        }
    }
}

pub(crate) fn send_verification_email(db_conn: &crate::DbConnection, account_id: &u32, email_address: &String) -> Option<Result<Response, Error>> {
    use crate::schema::{
        users::dsl::{
            users,
            userId,
            userName,
        },
        user_verification::dsl::{
            user_verification,
            userId as verification_userId,
            verificationCode,
            expire,
        },
    };

    info!("loading dotenv for mail");
    dotenv::dotenv().ok();

    info!("generating verification_info");
    let verification_id = super::super::generate_id(64);
    let verification_info = VerificationInfo::new(account_id, &verification_id, &(Utc::now() + Duration::days(1)).naive_utc());

    info!("verification_info generated");
    match diesel::replace_into(user_verification).values(verification_info).execute(&db_conn.0) {
        Ok(_insert_check) => {

            info!("verification_info inserted into database");
            let email = Message::builder()
                .from(std::env::var("MY_DEMON_LIST_NO_REPLY_EMAIL").unwrap().parse().unwrap())
                .to(email_address.parse().unwrap())
                .subject("Verify Your Account - MyDemonList")
                .body(format!("Click link to verify: {}/verify_account?verification_id={}", std::env::var("WEBSITE_BASE_URL").unwrap(), &verification_id))
                .unwrap();

            info!("authenticating into SMTP server");
            let creds = Credentials::new(std::env::var("SMTP_USERNAME").unwrap(), std::env::var("SMTP_PASSWORD").unwrap());
            // Open a remote connection to gmail
            let mailer = SmtpTransport::relay("smtp.gmail.com")
                .unwrap()
                .credentials(creds)
                .build();

            info!("authorized into SMTP sending email");
            // Send the email
            Some(mailer.send(&email))
        }
        Err(e) => {
            error!("database error with message: {}", e);
            None
        }
    }
}