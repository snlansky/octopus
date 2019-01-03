extern crate mysql;

mod table;
mod db;
mod config;
use config::DBRoute;


fn main() {
    let dbr = DBRoute {
        engine: String::from("Mysql"),
        user: String::from("snlan"),
        pass: String::from("snlan"),
        addr: String::from("www.snlan.top"),
        db: String::from("block"),
    };
    let mut db = db::open_db(dbr).unwrap();
    let res = db.load_db().unwrap();

    println!("{:#?}", db.tables);

}

