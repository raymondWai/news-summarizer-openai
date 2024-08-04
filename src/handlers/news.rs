use crate::{
    auth::ApiKey,
    db_utils::{batch_insert_articles, establish_connection},
    models::NewArticle,
    schema::{
        article::{self},
        source,
    },
};

use chrono::{NaiveDateTime, Timelike, Utc};
use diesel::{dsl::insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};
use newsdata_io_api::{
    apis::{GetLatestNewsParams, GetNewsSourcesParams, LatestNews, NewsSources},
    newsdata_io::{Auth, NewsdataIO},
    Error,
};
use serde_json::Value;
use std::{collections::HashMap, env, thread, time::Duration};

#[post("/update-news-lib")]
pub async fn update_news_lib_handler(_key: ApiKey<'_>) -> String {
    // Retrieve OpenAI API key from environment variables
    let newsdata_io_api_key = match env::var("NEWSDATA_IO_API_KEY") {
        Ok(api_key) => api_key,
        Err(e) => panic!("{}", e),
    };
    update_news_lib(newsdata_io_api_key).await;

    String::from("OK")
}

pub async fn update_news_lib(newsdata_io_api_key: String) -> Result<(), Error> {
    let mut conn = establish_connection();
    let articles: Vec<(String, NaiveDateTime)> = article::table
        .select((article::url, article::date))
        .order(article::date.desc())
        .load::<(String, NaiveDateTime)>(&mut conn)
        .unwrap();
    let now: NaiveDateTime = Utc::now().naive_utc();
    let latest_article_date = articles
        .first()
        .unwrap_or(&(String::from(""), now.with_hour(now.hour() - 2).unwrap()))
        .1;

    let sources: Vec<(String, i32)> = source::table
        .select((source::url, source::id))
        .distinct()
        .load::<(String, i32)>(&mut conn)
        .unwrap();
    let mut source_map = HashMap::<String, i32>::new();
    for source in sources {
        source_map.insert(source.0.clone(), source.1);
    }

    let newsdata_io_client = NewsdataIO::new(Auth::new(newsdata_io_api_key));
    let mut latest_news = newsdata_io_client
        .get_latest(&GetLatestNewsParams {
            country: Some(vec![
                String::from("gb"),
                String::from("us"),
                String::from("hk"),
            ]),
            size: Some(10),
            ..GetLatestNewsParams::default()
        })
        .unwrap();
    let mut next = match &latest_news["nextPage"] {
        serde_json::Value::String(value) => Some(String::from(value)),
        _ => None,
    };
    loop {
        let latest_news_results = match latest_news.clone()["results"].as_array() {
            Some(value) => value.to_owned(),
            None => break,
        };
        let mut article_to_add: Vec<NewArticle> = Vec::<NewArticle>::new();
        let mut on_error = false;
        for article in latest_news_results.iter() {
            let pub_date = match &article["pubDate"] {
                serde_json::Value::String(value) => {
                    NaiveDateTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S").unwrap()
                }
                _ => latest_article_date
                    .with_hour(latest_article_date.hour() - 1)
                    .unwrap(),
            };
            let title = match &article["title"] {
                serde_json::Value::String(value) => String::from(value),
                _ => String::from(""),
            };
            let url = match &article["link"] {
                serde_json::Value::String(value) => String::from(value),
                _ => String::from(""),
            };
            let keywords: Vec<Option<String>> = match &article["keywords"] {
                serde_json::Value::Array(value) => value
                    .iter()
                    .map(|keyword| match keyword {
                        serde_json::Value::String(value) => Some(value.to_owned()),
                        _ => None,
                    })
                    .collect(),
                _ => Vec::<Option<String>>::new(),
            };
            let creator: Vec<Option<String>> = match &article["creator"] {
                serde_json::Value::Array(value) => value
                    .iter()
                    .map(|c| match c {
                        serde_json::Value::String(value) => Some(value.to_owned()),
                        _ => None,
                    })
                    .collect(),
                _ => Vec::<Option<String>>::new(),
            };
            let video_source = match &article["video_url"] {
                serde_json::Value::String(value) => String::from(value),
                _ => String::from(""),
            };
            let description = match &article["description"] {
                serde_json::Value::String(value) => String::from(value),
                _ => String::from(""),
            };
            let content = match &article["content"] {
                serde_json::Value::String(value) => String::from(value),
                _ => String::from(""),
            };
            let image_url = match &article["image_url"] {
                serde_json::Value::String(value) => String::from(value),
                _ => String::from(""),
            };
            let country: Vec<Option<String>> = match &article["country"] {
                serde_json::Value::Array(value) => value
                    .iter()
                    .map(|c| match c {
                        serde_json::Value::String(value) => Some(value.to_owned()),
                        _ => None,
                    })
                    .collect(),
                _ => Vec::<Option<String>>::new(),
            };
            let source_id: Option<i32> = match &article["source_id"] {
                serde_json::Value::String(value) => match source_map.get(value) {
                    Some(value) => Some(value.to_owned()),
                    None => {
                        println!("Source not found: {}", url);
                        if on_error {
                            continue;
                        }
                        thread::sleep(Duration::from_millis(1000));
                        let source_res =
                            match newsdata_io_client.get_news_sources(&GetNewsSourcesParams {
                                id: match &article["article_id"] {
                                    serde_json::Value::String(value) => {
                                        Some(vec![String::from(value)])
                                    }
                                    _ => None,
                                },
                                ..GetNewsSourcesParams::default()
                            }) {
                                Ok(value) => Some(value),
                                Err(e) => {
                                    println!("Error: {}", e);
                                    on_error = true;
                                    None
                                }
                            };
                        match source_res {
                            Some(value) => {
                                let source = (match &value["results"] {
                                    serde_json::Value::Array(v) => {
                                        Some(v.first().unwrap().as_object().unwrap())
                                    }
                                    _ => None,
                                })
                                .unwrap();

                                let source_insert_res = insert_into(source::table)
                                    .values((
                                        source::name.eq(&source["name"].as_str().unwrap()),
                                        source::url.eq(&source["url"].as_str().unwrap()),
                                        source::country.eq(&source["country"]
                                            .as_array()
                                            .unwrap()
                                            .first()
                                            .unwrap()
                                            .as_str()
                                            .unwrap()),
                                        source::language.eq(&source["language"]
                                            .as_array()
                                            .unwrap()
                                            .first()
                                            .unwrap()
                                            .as_str()
                                            .unwrap()),
                                    ))
                                    .returning(source::id)
                                    .get_result::<i32>(&mut conn)
                                    .unwrap();
                                Some(source_insert_res)
                            }
                            None => None,
                        }
                    }
                },
                _ => None,
            };
            let language = match &article["language"] {
                serde_json::Value::String(value) => String::from(value),
                _ => String::from(""),
            };
            let category: Vec<Option<String>> = match &article["category"] {
                serde_json::Value::Array(value) => value
                    .iter()
                    .map(|c| match c {
                        serde_json::Value::String(value) => Some(value.to_owned()),
                        _ => None,
                    })
                    .collect(),
                _ => Vec::<Option<String>>::new(),
            };
            match source_id {
                Some(source) => {
                    println!("article_to_add: {}", title);
                    article_to_add.push(NewArticle {
                        title,
                        url,
                        keywords: Some(keywords),
                        creator: Some(creator),
                        video_source: Some(video_source),
                        description: description,
                        content: content,
                        date: pub_date,
                        image_url: Some(image_url),
                        source_id: source,
                        language: Some(language),
                        country: Some(country),
                        category: Some(category),
                        sentiment: None,
                        sentiment_stat: None,
                    });
                }
                _ => (),
            }
            if pub_date < latest_article_date {
                break;
            }
        }
        batch_insert_articles(&article_to_add, &mut conn);
        match next {
            None => break,
            Some(page) => {
                thread::sleep(Duration::from_millis(1000));
                latest_news = match next_page(&page, &newsdata_io_client) {
                    Ok(value) => value,
                    Err(e) => {
                        println!("Error: {}", e);
                        break;
                    }
                };
                next = match &latest_news["nextPage"] {
                    serde_json::Value::String(value) => Some(String::from(value)),
                    _ => None,
                }
            }
        };
    }
    Ok(())
}
fn next_page(next: &String, newsdata_io_client: &NewsdataIO) -> Result<Value, Error> {
    newsdata_io_client.get_latest(&GetLatestNewsParams {
        country: Some(vec![
            String::from("gb"),
            String::from("us"),
            String::from("hk"),
        ]),
        size: Some(10),
        page: Some((&next).to_string()),
        ..GetLatestNewsParams::default()
    })
}
