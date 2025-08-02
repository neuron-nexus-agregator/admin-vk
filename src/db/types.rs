#[derive(Debug)]
pub struct NewsSource {
    pub id: i32,
    pub vk: String,
    pub readable: String,
    pub is_rt: bool,
}
