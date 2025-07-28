use crate::admin::types;
use crate::config::admin;
use reqwest;

pub async fn send(data: &types::SendData) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .post(admin::ADMIN_URL)
        .header("X-Api-Key", admin::ADMIN_KEY)
        .header("Content-Type", admin::CONTENT_TYPE)
        .send()
        .await?;
    println!("Статус: {}", response.status());
    let body = response.text().await?;
    println!("Ответ: {}", body);
    Ok(())
}
