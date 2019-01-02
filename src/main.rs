mod table;

use table::Table;

fn main() {
    let pks = vec!["id"];
    let fns = vec!["id", "name", "age"];
    let fts = vec!["int", "vchar", "int"];

    let table = Table::new("kd", "user", pks, fns, fts);
    println!("{:?}", table);
    println!("{}", table.get_db_set_key());
    println!("{}", table.get_db_set_key());
}