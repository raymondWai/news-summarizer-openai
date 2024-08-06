use chrono::NaiveDate;
use diesel::{insert_into, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::{models::NewSummary, schema::summary};

pub fn insert_summary(new_summary: &String, conn: &mut PgConnection) -> usize {
    insert_into(summary::table)
        .values(&NewSummary {
            content: new_summary.clone(),
            date: chrono::Utc::now().naive_utc().date(),
        })
        .on_conflict(summary::date)
        .do_update()
        .set(summary::content.eq(new_summary))
        .execute(conn)
        .unwrap()
}

pub fn get_summary_by_date(date: &NaiveDate, conn: &mut PgConnection) -> String {
    use crate::schema::summary::content;
    summary::table
        .filter(summary::date.eq(date))
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
