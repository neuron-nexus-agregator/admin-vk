mod admin;
mod config;
mod model;
mod vk;

use crate::admin::sender;
use crate::admin::types;
use crate::config::admin as admin_config;
use crate::model::news::News;
use crate::vk::reader;
use dotenv::dotenv;
use tokio::sync::mpsc;
use tokio::task;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let (tx, mut rx) = mpsc::channel::<News>(100);
    let (admin_tx, admin_rx) = mpsc::channel::<types::SendData>(100);

    let group = "tassagency";
    let readable_name = "ТАСС";

    task::spawn(async move {
        reader::start(
            tx.clone(),
            group,
            readable_name,
            admin_config::RUSSIA_SOURCE,
        )
        .await;
    });

    task::spawn(async move {
        sender::start(admin_rx).await;
    });

    println!("Начинаем читать записи из группы {readable_name}\n\n");
    while let Some(news) = rx.recv().await {
        let send_data = types::SendData {
            title: "".to_string(),
            text: news.text,
            meta_title: "".to_string(),
            meta_description: "".to_string(),
            source: news.category,
            author: news.author,
            publish_t: Some(news.date),
        };
        match admin_tx.send(send_data).await {
            Ok(_) => println!("Запись отправлена"),
            Err(e) => println!("{e}"),
        }
    }
}
