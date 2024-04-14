use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Forecast {
    date: String,
    dateLabel: String,
    telop: String,
}

#[derive(Debug, Deserialize)]
struct ZipCloudResponse {
    forecasts: Vec<Forecast>,
}

pub async fn request(city: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = "https://weather.tsukumijima.net/api/forecast";
    let response = client
        .get(url)
        .query(&[("city", city)])
        .send()
        .await?;
    let body = response.json::<ZipCloudResponse>().await?;
    println!("{:?}", body);
    Ok(())
}
