#[derive(Clone, Debug)]
pub struct DBRoute {
    pub engine: String,
    pub user: String,
    pub pass: String,
    pub addr: String,
    pub db: String,
}