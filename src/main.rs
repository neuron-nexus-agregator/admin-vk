mod admin;
mod config;
mod model;
mod vk;

use crate::model::news::News;
use crate::vk::reader;
use chrono::{DateTime, Local, Utc};
use dotenv::dotenv;
use tokio::sync::mpsc;
use tokio::task;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let (tx, mut rx) = mpsc::channel::<News>(100);

    let group = "tassagency";
    let readable_name = "ТАСС";

    task::spawn(async move {
        reader::start(tx.clone(), group, readable_name).await;
    });

    println!("Начинаем читать записи из группы {readable_name}\n\n");
    while let Some(news) = rx.recv().await {
        let author = news.author;
        let text = news.text;
        let timestamp = news.date as i64;

        if let Some(datetime) = DateTime::<Utc>::from_timestamp(timestamp, 0) {
            let dt_local: DateTime<Local> = datetime.with_timezone(&Local);
            //println!("{dt_local} - {author}\n\n{text}\n\n------------------\n\n")
        } else {
            //println!("{author}\n\n{text}")
        }
    }
}
