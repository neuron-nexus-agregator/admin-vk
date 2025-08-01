mod admin;
mod config;
mod db;
mod model;
mod sentence_detector;
mod vk;

use crate::admin::sender;
use crate::admin::types;
use crate::config::admin as admin_config;
use crate::db::sources;
use crate::db::types::NewsSource;
use crate::model::news::News;
use crate::sentence_detector::detector::Detector;
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

    //TODO: получить источники из базы
    //TODO: в цикле запускать читателей для каждого источника

    match sources::get_sources().await {
        Err(e) => {
            // Exit
            eprintln!("Не удалось получить источники: {e}");
            std::process::exit(1);
        }
        Ok(sources) => {
            start_sources(sources, tx);
        }
    }

    task::spawn(async move {
        sender::start(admin_rx).await;
    });

    while let Some(news) = rx.recv().await {
        let (title, desc) = get_sentences(news.text.clone(), news.author.clone(), &detector);

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
            Err(e) => eprintln!("{e}"),
        }
    }
}

fn get_sentences(text: String, source: String, detector: &Detector) -> (String, String) {
    let sentences = detector.detect(&text);
    let mut title = "".to_string();
    let mut desc = "".to_string();

    if sentences.len() > 1 {
        title = sentences[0].to_string();
        desc = sentences[1].to_string();
    } else if sentences.len() == 1 {
        title = sentences[0].to_string();
    }

    if title.is_empty() {
        title = "Новость от ".to_string() + &source;
    }

    if desc.is_empty() {
        desc = "Необходимо вставить описание для новости".to_string()
    }

    (title, desc)
}

fn start_sources(sources: Vec<NewsSource>, tx: mpsc::Sender<News>) {
    for source in sources {
        let t = tx.clone();
        let gr = source.vk;
        let readable = source.readable;
        let cat = if source.is_rt {
            admin_config::TATARSTAN_SOURCE
        } else {
            admin_config::RUSSIA_SOURCE
        };

        let id = source.id;

        println!("Запускаю чтение из группы {readable} с ID {id}");

        task::spawn(async move {
            reader::start(t, gr.as_str(), readable.as_str(), cat).await;
        });
    }
}
