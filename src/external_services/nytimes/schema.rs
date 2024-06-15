use rocket::serde::Deserialize;

#[derive(Deserialize)]
pub struct Article {
    pub uri: Option<String>,
    pub url: Option<String>,
    pub id: u64,
    pub asset_id: u64,
    pub source: Option<String>,
    pub published_date: Option<String>,
    pub updated: Option<String>,
    pub section: Option<String>,
    pub subsection: Option<String>,
    pub nytdsection: Option<String>,
    pub adx_keywords: Option<String>,
    pub byline: Option<String>,
    pub column: Option<String>,
    pub title: Option<String>,
    #[serde(alias = "abstract")]
    pub _abstract: Option<String>,
    pub des_facet: Option<Vec<String>>,
    pub geo_facet: Option<Vec<String>>,
    pub org_facet: Option<Vec<String>>,
    pub per_facet: Option<Vec<String>>,
    pub eta_id: Option<u32>,
}

#[derive(Deserialize)]
pub struct MostPopularNewsResponse {
    pub status: Option<String>,
    pub copyright: Option<String>,
    pub num_results: i32,
    pub results: Vec<Article>,
}
