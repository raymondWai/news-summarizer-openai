mod external_services;
pub mod handlers;
pub mod models;
pub mod schema;
mod utils;

use dotenvy::dotenv;
use handlers::{get_news, summarize_single_text};
#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "OK"
}

#[launch]
fn rocket() -> _ {
    let _ = dotenv();
    rocket::build().mount("/", routes![index, summarize_single_text, get_news])
}
