use std::env;

use super::schema::MostPopularNewsResponse;

pub async fn get_most_popular(period: i8) -> MostPopularNewsResponse {
    let nytimes_api_key = match env::var("NYTIMES_API_KEY") {
        Ok(api_key) => api_key,
        Err(e) => panic!("{}", e),
    };
    let res = reqwest::get(format!(
        "https://api.nytimes.com/svc/mostpopular/v2/viewed/{}.json?api-key={}",
        period, nytimes_api_key
    ))
    .await
    .unwrap();
    let res = res.json::<MostPopularNewsResponse>().await.unwrap();
    res
}
