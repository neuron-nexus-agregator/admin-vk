mod admin;
mod config;
mod model;
mod telegram;
mod vk;
use std::fs::read;

use dotenv::dotenv;
// use crate::admin::sender;
// use crate::admin::types as admin_types;
// use crate::config::admin as admin_config;
use crate::vk::reader;

#[tokio::main]
async fn main() {
    dotenv().ok();

    match reader::read_posts("vk", "1").await {
        Ok(posts) => {
            for item in posts.response.items {
                println!("{}", item.text);
            }
        }
        Err(e) => {
            println!("{e}");
        }
    }
}
