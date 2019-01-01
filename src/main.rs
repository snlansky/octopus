mod table;
use table::Table;

fn main(){
    let pks = vec!["id"];
    let mut ps = Vec::new();
    for &pk in pks.iter() {
        ps.push(String::from(pk));
    }

    let mut fns = Vec::new();
    for name in vec!["id", "name", "age"] {
        fns.push(String::from(name))
    }


    let mut fts = Vec::new();
    for name in vec!["int", "vchar", "int"] {
        fts.push(String::from(name))
    }

    let table = Table::new("kd", "user", ps, fns, fts);
    println!("{:?}", table);
    println!("{}", table.get_db_set_key());
    println!("{}", table.get_db_set_key());
}