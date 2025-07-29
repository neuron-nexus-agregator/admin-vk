use crate::config::vk;
use crate::model::news::News;
use crate::vk::types;
use reqwest;
use std::env;
use tokio::sync::mpsc;
use tokio::time::{Duration, sleep};

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

pub async fn start(tx: mpsc::Sender<News>, group: &str, readable_name: &str) {
    let mut last_read: u64 = 0;
    let readable_string = readable_name.to_string();
    let sleep_secs = 60 * 5;
    loop {
        if last_read == 0 {
            match read_posts(group, "1").await {
                Ok(result) => {
                    if !result.response.items.is_empty() {
                        let date = result.response.items[0].date;
                        last_read = date;
                    }
                }
                Err(e) => {
                    println!("{e}");
                }
            }
        } else {
            match read_posts(group, "10").await {
                Err(e) => {
                    println!("{e}");
                }
                Ok(result) => {
                    let new_posts = get_new_posts(&result, last_read);
                    if !new_posts.is_empty() {
                        for post in new_posts {
                            let news = News {
                                text: post,
                                author: readable_string.clone(),
                            };
                            if let Err(e) = tx.send(news).await {
                                println!("{e}");
                            }
                        }
                    }
                    last_read = result.response.items[0].date;
                }
            }
        }

        sleep(Duration::from_secs(sleep_secs)).await;
    }
}
