use serde::Deserialize;

#[derive(Deserialize)]
pub struct Item {
    pub text: String,
    pub date: u64,
    pub is_pinned: Option<u8>,
}

#[derive(Deserialize)]
pub struct InternalResponse {
    pub items: Vec<Item>,
}

#[derive(Deserialize)]
pub struct Response {
    pub response: InternalResponse,
}
