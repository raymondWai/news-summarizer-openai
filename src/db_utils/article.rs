use std::error::Error;

use diesel::{
    insert_into, BelongingToDsl, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
    SelectableHelper,
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

pub fn get_article_by_region(
    countries: &Vec<String>,
    conn: &mut PgConnection,
) -> Result<Vec<Article>, Box<dyn Error + Send + Sync>> {
    let sources = source::table
        .filter(source::country.eq_any(countries))
        .select(Source::as_select())
        .load(conn)?;
    Ok(Article::belonging_to(&sources)
        .select(Article::as_select())
        .load(conn)?)
}
