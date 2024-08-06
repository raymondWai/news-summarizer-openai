use chrono::{Datelike, NaiveDate, Utc};
use diesel::{QueryDsl, RunQueryDsl};

use crate::db_utils::{
    establish_connection, get_article_by_region_date, get_summary_by_date, insert_summary,
};
use crate::external_services;
use crate::schema::source;
use std::env;
use std::fmt::Write;

#[get("/summary/nytimes")]
// Rocket route that handles requests to '/summarize'
pub async fn summarize_nytimes() -> String {
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
        Some(1.0),           // Temperature
        Some("gpt-4-turbo"), // Model
        Some(""),            // System message
        Some(10000),         // Max tokens
        Some(","),           // Stop sequence
        Some(true),          // Summarize recursively
        Some(true),          // Return full text
    )
    .await;

    // Return the summarized response
    summarized_response.clone()
}

#[get("/summary?<date>")]
pub async fn summarize_date(date: Option<&str>) -> String {
    let now = Utc::now().naive_utc();
    let date = match date {
        Some(s) => NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap_or_else(|_| {
            // Handle parsing error, e.g., use a default FixedOffset
            NaiveDate::from_ymd_opt(now.year(), now.month(), now.day()).unwrap()
            // Example: UTC+00:00
        }),
        None => NaiveDate::from_ymd_opt(now.year(), now.month(), now.day()).unwrap(), // Example: UTC+00:00
    };
    get_summary_by_date(&date, &mut establish_connection())
}

#[post("/generate-daily-summary")]
pub async fn generate_daily_summary() -> String {
    let now: chrono::NaiveDateTime = Utc::now().naive_utc();
    let mut conn = establish_connection();
    let sources: Vec<String> = source::table
        .select(source::country)
        .distinct()
        .load::<String>(&mut conn)
        .unwrap();
    let articles = get_article_by_region_date(
        &sources,
        &(NaiveDate::from_ymd_opt(now.year(), now.month(), 5).unwrap()),
        &mut conn,
    );
    if let Ok(cahced_articles) = articles {
        // Initialize prompt for summarization
        let mut prompt = String::from(
            "Summarize following articles in 200 words or less.\nProvided that --- is separator between articles.---\n",
        );
        // Iterate over articles, appending abstracts to prompt
        for article in cahced_articles.iter() {
            write!(
                &mut prompt,
                "---\nContent: {}\n Region: {}\n---\n",
                &article.description,
                (&article
                    .country
                    .clone()
                    .unwrap_or(vec![Some(String::from(""))]))
                    .iter()
                    .map(|c| c.clone().unwrap_or(String::from("")))
                    .collect::<Vec<String>>()
                    .join(", ")
            )
            .unwrap();
        }
        println!("prompt: {}", &prompt);

        // Call OpenAI API to summarize the text
        let summarized_response = external_services::openai::summarize(
            &prompt,
            Some(1.0),           // Temperature
            Some("gpt-4o-mini"), // Model
            Some(""),            // System message
            Some(10000),         // Max tokens
            Some(","),           // Stop sequence
            Some(true),          // Summarize recursively
            Some(true),          // Return full text
        )
        .await;

        insert_summary(&summarized_response, &mut conn);
    }

    String::from("OK")
}
