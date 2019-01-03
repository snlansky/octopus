#[derive(Clone, Debug)]
pub struct DBRoute {
    pub engine: String,
    pub user: String,
    pub pass: String,
    pub addr: String,
    pub db: String,
}

//impl Copy for DBRoute {
//
//}

pub enum Engine {
    Mysql,
    Postgres,
}