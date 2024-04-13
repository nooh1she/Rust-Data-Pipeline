use reqwest::Client;
use std::error::Error;

pub async fn request(city: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = "https://weather.tsukumijima.net/api/forecast";
    let response = client
        .get(url)
        .query(&[("city", city)])
        .send()
        .await?;
    let body = response.text().await?;
    println!("{}", body);
    Ok(())
}