#[macro_use]
extern crate diesel;
use diesel::*;
use diesel::prelude::*;
#[macro_use]
extern crate serde_derive;

extern crate dotenv;

pub mod github;
pub mod gitlab;
pub mod kitchensink;
pub mod models;
pub mod schema;

use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}