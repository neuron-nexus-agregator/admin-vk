use crate::config::vk;
use crate::model::news::News;
use crate::vk::types;
use std::env;
use tokio::sync::mpsc;
use tokio::time::{Duration, sleep};

pub async fn read_posts(url: &str, limit: &str) -> Result<types::Response, reqwest::Error> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (compatible; MyVkBot/1.0)")
        .build()
        .unwrap();

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

pub fn get_new_posts(result: &types::Response, last_read: u64) -> Vec<&types::Item> {
    println!("Начинаем проверять посты с {last_read}");
    let mut filtered_posts: Vec<&types::Item> = vec![];
    for item in result.response.items.iter() {
        if item.date > last_read && !item.text.is_empty() {
            let text = item.text.to_string();
            let date = item.date;
            println!("Найден подходящий пост с текстом: {text} и timestamp: {date} > {last_read}");
            filtered_posts.push(item);
        } else if let Some(d) = item.is_pinned
            && d != 1
        {
            break;
        }
    }
    filtered_posts
}

pub async fn start(tx: mpsc::Sender<News>, group: &str, readable_name: &str, category: u8) {
    println!("Пытаюсь читать группу {readable_name}");
    let mut last_read: u64 = 0;
    let readable_string = readable_name.to_string();
    let sleep_secs = 60 * 5; // 5 минут
    loop {
        match read_posts(group, "10").await {
            Ok(result) => {
                if last_read == 0 {
                    if result.response.items.len() > 1 {
                        last_read = std::cmp::max(
                            result.response.items[0].date,
                            result.response.items[1].date,
                        )
                    } else if result.response.items.len() == 1 {
                        last_read = result.response.items[0].date;
                    }
                } else {
                    for post in get_new_posts(&result, last_read) {
                        let url = format!(
                            "\n\n\nИсточник: https://vk.com/{group}?w=wall{}_{}",
                            post.owner_id, post.id
                        );
                        let news = News {
                            text: post.text.clone() + &url,
                            author: readable_string.clone(),
                            date: post.date,
                            category,
                        };
                        if let Err(e) = tx.send(news).await {
                            eprintln!("{e}");
                        }
                        last_read = last_read.max(post.date);
                    }
                }
            }
            Err(err) => println!("Не удалось прочитать посты: {err}"),
        }

        println!("{readable_name} отправляется спать на {sleep_secs} секунд");
        sleep(Duration::from_secs(sleep_secs)).await;
    }
}
