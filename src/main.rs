mod external_services;
mod utils;

use dotenvy::dotenv;
use std::env;
#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "OK"
}

#[get("/summarize")]
async fn summarize_single_text() -> String {
    let openai_api_key = match env::var("OPENAI_API_KEY") {
        Ok(api_key) => api_key,
        Err(e) => panic!("{}", e),
    };
    println!("{}", openai_api_key);
    let res = external_services::nytimes::get_most_popular(1).await;
    let articles = &res.results;
    let mut prompt = String::from("Rewrite this text in summarized form.");
    for article in articles.iter() {
        let url = match &article.url {
            Some(value) => value,
            None => "",
        };
        let _abstract = match &article._abstract {
            Some(value) => value,
            None => "",
        };
        prompt.push_str(_abstract);
        println!("{}: {}", url, &_abstract);
    }

    let summarized_response = external_services::openai::summarize(
        &prompt,
        Some(1.0),
        Some("gpt-4-turbo"),
        Some(""),
        Some(10000),
        Some(","),
        Some(true),
        Some(true),
    )
    .await;

    summarized_response.clone()
}

#[launch]
fn rocket() -> _ {
    dotenv().expect(".env file not found");
    rocket::build().mount("/", routes![index, summarize_single_text])
}
