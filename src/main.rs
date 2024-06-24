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
// Rocket route that handles requests to '/summarize'
async fn summarize_single_text() -> String {
    // Retrieve OpenAI API key from environment variables
    let openai_api_key = match env::var("OPENAI_API_KEY") {
        Ok(api_key) => api_key,
        Err(e) => panic!("{}", e),
    };
    println!("{}", openai_api_key);

    // Fetch most popular articles from NYTimes API
    let res = external_services::nytimes::get_most_popular(1).await;
    let articles = &res.results;

    // Initialize prompt for summarization
    let mut prompt = String::from("Rewrite this text in summarized form.");

    // Iterate over articles, appending abstracts to prompt
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

    // Call OpenAI API to summarize the text
    let summarized_response = external_services::openai::summarize(
        &prompt,
        Some(1.0), // Temperature
        Some("gpt-4-turbo"), // Model
        Some(""), // System message
        Some(10000), // Max tokens
        Some(","), // Stop sequence
        Some(true), // Summarize recursively
        Some(true), // Return full text
    )
    .await;

    // Return the summarized response
    summarized_response.clone()
}


#[launch]
fn rocket() -> _ {
    let _ = dotenv();
    rocket::build().mount("/", routes![index, summarize_single_text])
}
