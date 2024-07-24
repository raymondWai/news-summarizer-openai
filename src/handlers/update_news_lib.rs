use newsdata_io_api::{
    apis::{GetLatestNewsParams, LatestNews},
    newsdata_io::{Auth, NewsdataIO},
};
use std::env;

#[get("/update-news-lib")]
pub async fn get_news() -> String {
    // Retrieve OpenAI API key from environment variables
    let newsdata_io_api_key = match env::var("NEWSDATA_IO_API_KEY") {
        Ok(api_key) => api_key,
        Err(e) => panic!("{}", e),
    };
    let newsdata_io_client = NewsdataIO::new(Auth::new(newsdata_io_api_key));
    let params = GetLatestNewsParams {
        country: Some(vec![String::from("gb"), String::from("us")]),
        ..GetLatestNewsParams::default()
    };

    let latest_news = newsdata_io_client.get_latest(&params).unwrap();
    println!("{}", latest_news);

    String::from("OK")
}
