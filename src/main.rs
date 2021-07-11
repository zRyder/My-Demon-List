#![feature(proc_macro_hygiene, decl_macro)]
#![feature(in_band_lifetimes)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate tokio;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate dotenv;
#[macro_use] extern crate log;
#[macro_use] extern crate log4rs;

embed_migrations!();

use std::env;

use rocket_contrib::databases::diesel as rocket_diesel;
use rocket_contrib::serve::StaticFiles;

use model::{
    geometry_dash,
    users,
    rating,
};

use diesel::Connection;

use log::{debug, error, info, trace, warn, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log4rs::filter::threshold::ThresholdFilter;
use log4rs::append::rolling_file::RollingFileAppender;

pub mod model;
pub mod schema;

#[database("mysql_db")]
pub struct DbConnection(rocket_diesel::mysql::MysqlConnection);

fn main() {
    println!("hi");
    let fixed_window_roller =
        FixedWindowRoller::builder().build("logfile.{}.log",100000).unwrap();
    let size_trigger = SizeTrigger::new(10485760);
    let compound_policy = CompoundPolicy::new(Box::new(size_trigger),Box::new(fixed_window_roller));

    let config = Config::builder()
        .appender(
            Appender::builder()
                .build(
                    "logfile.log",
                    Box::new(
                        RollingFileAppender::builder()
                            .encoder(Box::new(PatternEncoder::new("{d} {l}::{m}{n}")))
                            .build("logs/logfile.log", Box::new(compound_policy)).unwrap(),
                    ),
                ),
        )
        .build(
            Root::builder()
                .appender("logfile.log")
                .build(LevelFilter::Debug),
        ).unwrap();

    let _handle = log4rs::init_config(config).unwrap();

    info!("Loading env variables");
    dotenv::dotenv().ok();

    let connection = diesel::mysql::MysqlConnection::establish(&env::var("DATABASE_URL").unwrap()).unwrap();
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout());

    rocket::ignite()
        .mount("/",StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static/")))
        .mount("/api/gd", routes![geometry_dash::routes::search])
        .mount("/api/users", routes![users::routes::create_user, users::routes::login_user, users::routes::update_username, users::routes::update_password])
        .mount("/api/rating", routes![rating::routes::rate_level, rating::routes::get_level_rating])
        .attach(DbConnection::fairing())
        .launch();
}
