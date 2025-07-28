use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct SendData {
    pub title: String,
    pub text: String,
    pub meta_title: String,
    pub meta_description: String,
    pub source: u8,
    pub author: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_t: Option<u64>,
}

#[derive(Deserialize)]
pub struct ReadData {
    pub id: String,
}
