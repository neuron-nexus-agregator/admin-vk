mod admin;
mod config;
use crate::admin::sender;
use crate::admin::types;
use crate::config::admin as admin_config;

#[tokio::main]
async fn main() {
    let req = &types::SendData {
        title: "Test".to_string(),
        text: "Test text".to_string(),
        meta_description: "Test description".to_string(),
        meta_title: "Test title".to_string(),
        source: admin_config::RUSSIA_SOURCE,
        author: "Тест".to_string(),
        publish_t: None,
    };
    match sender::send(req).await {
        Ok(id) => println!("Success with id: {id}"),
        Err(e) => println!("{e}"),
    }
}
