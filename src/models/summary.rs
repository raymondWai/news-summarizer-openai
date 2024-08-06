use chrono::NaiveDate;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::summary)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Summary {
    pub id: i32,
    pub content: String,
    pub date: NaiveDate,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::summary)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSummary {
    pub content: String,
    pub date: NaiveDate,
}
