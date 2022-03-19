extern crate diesel;
extern crate diequery;

use diesel::prelude::*;
use diequery::connection::establish_connection;
use diequery::models::*;
use diequery::schema::posts::dsl::*;

fn main() {
    let conn = establish_connection();
    let results = posts
        .filter(published.eq(false))
        .limit(5)
        .load::<Post>(&conn)
        .expect("Error loading posts");
    for ele in results {
        println!("{:?}", ele);
    }
}
