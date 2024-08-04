use chrono::{Datelike, NaiveDateTime, Timelike};
use diesel::{insert_into, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use rocket::tokio;

use crate::db_utils::establish_connection;
use crate::handlers::update_news_lib;
use crate::{generate_daily_summary, models::NewSummary, schema::summary};

pub fn insert_summary(new_summary: &String, conn: &mut PgConnection) -> usize {
    insert_into(summary::table)
        .values(&NewSummary {
            content: new_summary.clone(),
            date: chrono::Utc::now().naive_utc(),
        })
        .on_conflict(summary::date)
        .do_update()
        .set(summary::content.eq(new_summary))
        .execute(conn)
        .unwrap()
}

pub fn get_summary_by_date(date: &NaiveDateTime, conn: &mut PgConnection) -> String {
    use crate::schema::summary::content;
    let mut summary = String::from("");
    summary::table
        .filter(
            summary::date.ge(date
                .with_hour(0)
                .unwrap()
                .with_minute(0)
                .unwrap()
                .with_second(0)
                .unwrap()),
        )
        .filter(
            summary::date.le(date
                .with_day(date.day() + 1)
                .unwrap()
                .with_hour(0)
                .unwrap()
                .with_minute(0)
                .unwrap()
                .with_second(0)
                .unwrap()),
        )
        .select(content)
        .load::<String>(conn)
        .unwrap()
        .first()
        .unwrap_or(&String::from(format!(
            "The summary of {} is not available.",
            date.format("%Y-%m-%d")
        )))
        // .unwrap_or_else(|| {
        //     tokio::runtime::Builder::new_current_thread()
        //         .build()
        //         .unwrap()
        //         .block_on(async {
        //             use std::env;
        //             let newsdata_io_api_key = match env::var("NEWSDATA_IO_API_KEY") {
        //                 Ok(api_key) => api_key,
        //                 Err(e) => panic!("{}", e),
        //             };
        //             let _ = update_news_lib(newsdata_io_api_key).await;
        //             generate_daily_summary().await;
        //         });
        //     summary = get_summary_by_date(&date, &mut establish_connection());
        //     &summary
        // })
        .to_owned()
}
