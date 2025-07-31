mod admin;
mod config;
mod model;
mod sentence_decetcor;
mod vk;

use crate::admin::sender;
use crate::admin::types;
use crate::config::admin as admin_config;
use crate::model::news::News;
use crate::sentence_decetcor::detector::Detector;
use crate::vk::reader;
use dotenv::dotenv;
use tokio::sync::mpsc;
use tokio::task;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let detector = Detector::new();
    let (tx, mut rx) = mpsc::channel::<News>(100);
    let (admin_tx, admin_rx) = mpsc::channel::<types::SendData>(100);

    let group = "tassagency";
    let readable_name = "ТАСС";

    task::spawn(async move {
        reader::start(
            tx.clone(),
            group,
            readable_name,
            admin_config::TATARSTAN_SOURCE,
        )
        .await;
    });

    task::spawn(async move {
        sender::start(admin_rx).await;
    });

    println!("Начинаем читать записи из группы {readable_name}\n\n");
    while let Some(news) = rx.recv().await {
        let sentences = detector.detect(&news.text);
        let mut title = "".to_string();
        let mut desc = "".to_string();

        if sentences.len() > 1 {
            title = sentences[0].to_string();
            desc = sentences[1].to_string();
        } else if sentences.len() == 1 {
            title = sentences[0].to_string();
        }

        if title.is_empty() {
            title = "Новость от ".to_string() + &news.author;
        }

        if desc.is_empty() {
            desc = "Необходимо вставить описание для новости".to_string()
        }

        let send_data = types::SendData {
            title: title.clone(),
            text: news.text,
            meta_title: title.clone(),
            meta_description: desc,
            source: news.category,
            author: news.author + " - ВК",
            publish_t: Some(news.date),
        };
        match admin_tx.send(send_data).await {
            Ok(_) => println!("Запись отправлена"),
            Err(e) => println!("{e}"),
        }
    }
}
