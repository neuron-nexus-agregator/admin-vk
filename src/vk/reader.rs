use crate::config::vk;
use crate::vk::types;
use reqwest;

pub async fn read_posts(url: &str, limit: &str) -> Result<types::Response, reqwest::Error> {
    let client = reqwest::Client::new();

    let response_url = client
        .get(vk::GET_URL)
        .query(&[
            ("domain", url),
            ("count", limit),
            ("access_token", vk::ACCESS_TOKEN),
            ("v", vk::API_VERSION),
        ])
        .send()
        .await?;
    let response: types::Response = response_url.json().await?;

    Ok(response)
}
