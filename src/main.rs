mod auth;
mod db_utils;
mod external_services;
mod handlers;
mod models;
mod schema;
mod utils;

use dotenvy::dotenv;
use handlers::{
    generate_daily_summary, summarize_date, summarize_nytimes, update_news_lib_handler,
};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "OK"
}

#[launch]
fn rocket() -> _ {
    let _ = dotenv();
    rocket::build().mount(
        "/",
        routes![
            index,
            summarize_nytimes,
            update_news_lib_handler,
            summarize_date,
            generate_daily_summary
        ],
    )
}
