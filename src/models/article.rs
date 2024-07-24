use crate::models::Source;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Source, foreign_key = source_id))]
#[diesel(table_name = crate::schema::article)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub keywords: Option<Vec<Option<String>>>,
    pub creator: Option<Vec<Option<String>>>,
    pub video_source: Option<String>,
    pub description: String,
    pub content: String,
    pub date: NaiveDateTime,
    pub image_url: Option<String>,
    pub source_id: i32,
    pub language: Option<String>,
    pub country: Option<Vec<Option<String>>>,
    pub category: Option<Vec<Option<String>>>,
    pub sentiment: Option<String>,
    pub sentiment_stat: Option<serde_json::Value>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::article)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewArticle<'a> {
    pub title: &'a str,
    pub url: &'a str,
    pub keywords: Option<Vec<Option<&'a str>>>,
    pub creator: Option<Vec<Option<&'a str>>>,
    pub video_source: Option<&'a str>,
    pub description: &'a str,
    pub content: &'a str,
    pub date: NaiveDateTime,
    pub image_url: Option<&'a str>,
    pub source_id: i32,
    pub language: Option<&'a str>,
    pub country: Option<Vec<Option<&'a str>>>,
    pub category: Option<Vec<Option<&'a str>>>,
    pub sentiment: Option<&'a str>,
    pub sentiment_stat: Option<serde_json::Value>,
}
