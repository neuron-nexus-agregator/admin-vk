use crate::admin::types;
use crate::config::admin;
use reqwest;
use std::env;

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
