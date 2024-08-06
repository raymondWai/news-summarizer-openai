use std::error::Error;

use chrono::NaiveDate;
use diesel::{
    dsl::sql,
    insert_into,
    sql_types::{Bool, Date},
    BelongingToDsl, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};

use crate::{
    models::{Article, NewArticle, Source},
    schema::{article, source},
};

pub fn batch_insert_articles(articles: &Vec<NewArticle>, conn: &mut PgConnection) -> usize {
    insert_into(article::table)
        .values(articles)
        .on_conflict_do_nothing()
        .execute(conn)
        .unwrap()
}

pub fn get_article_by_region_date(
    countries: &Vec<String>,
    date: &NaiveDate,
    conn: &mut PgConnection,
) -> Result<Vec<Article>, Box<dyn Error + Send + Sync>> {
    let sources = source::table
        .filter(source::country.eq_any(countries))
        .select(Source::as_select())
        .load(conn)?;
    let articles_query = Article::belonging_to(&sources)
        .select(Article::as_select())
        .filter(sql::<Bool>("date::timestamp::date = ").bind::<Date, _>(date));
    Ok(articles_query.load(conn)?)
}
