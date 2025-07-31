use crate::admin::types;
use crate::config::admin;
use std::env;
use tokio::sync::mpsc;

pub async fn send(data: &types::SendData) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .post(admin::ADMIN_URL)
        .header(
            "X-Api-Key",
            env::var("ADMIN_KEY").expect("No key for admin"),
        )
        .header("Content-Type", admin::CONTENT_TYPE)
        .json(data)
        .send()
        .await?;

    let res_data: types::ReadData = response.json().await?;

    Ok(res_data.id)
}

pub async fn start(mut rx: mpsc::Receiver<types::SendData>) {
    while let Some(data) = rx.recv().await {
        match send(&data).await {
            Ok(id) => {
                println!("Sent data with id {id}");
            }
            Err(e) => {
                eprintln!("Error sending data: {e}");
            }
        }
    }
}
