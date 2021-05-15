#![feature(proc_macro_hygiene, decl_macro)]
#![feature(in_band_lifetimes)]


#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate futures;
#[macro_use] extern crate tokio;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;

embed_migrations!();

use std::error::Error;

use futures::executor::block_on;
use rocket_contrib::databases::diesel as rocket_diesel;
use rocket_contrib::serve::StaticFiles;
use tokio::runtime::Runtime;

use model::{
    geometry_dash,
    users,
};
use diesel::Connection;

pub mod model;
pub mod schema;

#[database("mysql_db")]
pub struct DbConnection(rocket_diesel::mysql::MysqlConnection);

fn main()
{
    let connection = diesel::mysql::MysqlConnection::establish("mysql://root:Rayuwwe6@127.0.0.1:3306/my_demon_list_schema").unwrap();
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout());

    // test::test();
    rocket::ignite()
        .mount("/",StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static/")))
        .mount("/api", routes![geometry_dash::routes::search, users::routes::create_user, users::routes::login_user])
        .attach(DbConnection::fairing())
        .launch();
}
