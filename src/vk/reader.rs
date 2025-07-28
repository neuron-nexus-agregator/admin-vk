use crate::config::vk;
use crate::vk::types;
use reqwest;
use std::env;

pub async fn read_posts(url: &str, limit: &str) -> Result<types::Response, reqwest::Error> {
    let client = reqwest::Client::new();

    let response_url = client
        .get(vk::GET_URL)
        .query(&[
            ("domain", url),
            ("count", limit),
            (
                "access_token",
                &env::var("ACCESS_TOKEN").expect("No VK Access Token"),
            ),
            ("v", vk::API_VERSION),
        ])
        .send()
        .await?;
    let response: types::Response = response_url.json().await?;

    Ok(response)
}

pub fn get_new_posts(result: &types::Response, last_read: u64) -> Vec<String> {
    let mut filtered_posts: Vec<_> = result
        .response
        .items
        .iter()
        .filter(|post| post.date > last_read)
        .collect();

    // Сортируем по возрастанию date
    filtered_posts.sort_by_key(|post| post.date);

    filtered_posts
        .into_iter()
        .map(|post| post.text.clone())
        .collect()
}
