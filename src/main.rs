#![feature(proc_macro_hygiene, decl_macro)]
#![feature(in_band_lifetimes)]


#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate futures;
#[macro_use] extern crate tokio;
#[macro_use] extern crate diesel;

use std::error::Error;

use futures::executor::block_on;
use rocket_contrib::databases::diesel as rocket_diesel;
use rocket_contrib::serve::StaticFiles;
use tokio::runtime::Runtime;

pub mod model;
pub mod test;
pub mod routes;
pub mod schema;

#[database("mysql_db")]
pub struct DbConnection(rocket_diesel::mysql::MysqlConnection);

fn main()
{
    // test::test();
    rocket::ignite()
        .mount("/",StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static/")))
        .mount("/api", routes![routes::search, routes::create_user])
        .attach(DbConnection::fairing())
        .launch();
}
