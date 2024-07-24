use diesel::prelude::*;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::source)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Source {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub country: String,
    pub language: String,
}
