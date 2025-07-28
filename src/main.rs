mod admin;
mod config;
mod vk;
use dotenv::dotenv;
// use crate::admin::sender;
// use crate::admin::types as admin_types;
// use crate::config::admin as admin_config;
// use crate::vk::reader;

#[tokio::main]
async fn main() {
    dotenv().ok();
}
