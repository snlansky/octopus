extern crate redis;
use mem::redis::Commands;

pub struct Mem {}

impl Mem {
    pub fn fetch_an_integer(param: &str) -> redis::RedisResult<isize> {
        // connect to redis
        let client = redis::Client::open(param)?;
        let con = client.get_connection()?;
        // throw away the result, just make sure it does not fail
        let _: () = con.set("my_key", 42)?;
        con.get("my_key")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redis() {
        let res = Mem::fetch_an_integer("redis://:snlan@www.snlan.top:6379/");
        match res {
            Ok(t) => println!("{}", t),
            Err(e)=> println!("err {}", e),
        }
    }
}