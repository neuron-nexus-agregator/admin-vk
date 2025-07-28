use serde::Serialize;

#[derive(Serialize)]
pub struct SendData {
    pub title: String,
    pub text: String,
    pub meta_title: String,
    pub meta_description: String,
    pub source: u8,
    pub publish_t: u64,
}

pub struct ReadData {
    pub id: i64,
}
