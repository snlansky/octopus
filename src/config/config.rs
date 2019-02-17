#[derive(Clone, Debug)]
pub struct DBRoute {
    pub engine: String,
    pub user: String,
    pub pass: String,
    pub addr: String,
    pub db: String,
}


#[derive(Clone, Debug)]
pub struct MemRoute {
    pub host: String,
    pub port: i32,
    pub db: i32,
    pub expire: i64,
    pub pass: String,
}

#[derive(Clone, Debug)]
pub struct ZKS {
    pub path: String,
    pub servers: Vec<String>,
    pub buffer: i32,
    pub multiple: i32,
}

pub fn init(file: String) {}