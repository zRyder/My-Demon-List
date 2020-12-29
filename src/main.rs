#![feature(proc_macro_hygiene, decl_macro)]
#![feature(in_band_lifetimes)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate futures;
#[macro_use] extern crate tokio;

use futures::executor::block_on;
use tokio::runtime::Runtime;

use rocket_contrib::serve::
{
    StaticFiles
};
use std::error::Error;

pub mod model;
pub mod gd;
pub mod test;
pub mod routes;


fn main()
{

    // test::test();
    rocket::ignite()
        .mount("/",StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static/")))
        .mount("/", routes![routes::search])
        .launch();
}
