#![feature(proc_macro_hygiene, decl_macro)]
#![feature(in_band_lifetimes)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate tokio;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate dotenv;
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

pub mod model;
pub mod schema;

#[database("mysql_db")]
pub struct DbConnection(rocket_diesel::mysql::MysqlConnection);

fn main() {
    dotenv::dotenv().ok();

    let connection = diesel::mysql::MysqlConnection::establish(&env::var("DATABASE_URL").unwrap()).unwrap();
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout());

    rocket::ignite()
        .mount("/",StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static/")))
        .mount("/api/gd", routes![geometry_dash::routes::search])
        .mount("/api/users", routes![users::routes::create_user, users::routes::login_user, users::routes::update_username])
        .mount("/api/rating", routes![rating::routes::rate_level, rating::routes::get_level_rating])
        .attach(DbConnection::fairing())
        .launch();
}
