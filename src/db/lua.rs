extern crate redis;
extern crate sha1;

use self::redis::{Connection, ToRedisArgs, cmd};
use self::redis::FromRedisValue;
use self::redis::ConnectionLike;
use self::redis::RedisResult;
use self::redis::ErrorKind;
use self::sha1::Sha1;

pub struct LuaScript<'a> {
    statements: Vec<&'a str>,
    keys: Vec<String>,
    key_index: i32,
    argv: Vec<Vec<u8>>,
    arg_index: i32,

}

impl <'a> LuaScript<'a>  {
    pub fn new() -> LuaScript<'a>  {
        LuaScript {
            statements: vec!["local total = 0", "local r = 0"],
            keys: Vec::new(),
            key_index: 1,
            argv: Vec::new(),
            arg_index: 1,
        }
    }

    pub fn append(&mut self, code: &'a str) {
        self.statements.push(code)
    }

    pub fn arg<T: ToRedisArgs>(&mut self, arg: T) {
        arg.write_redis_args(&mut self.argv);
//        self.argv.push(arg);
    }
    pub fn invoke(self, con: &Connection) -> Result<isize, redis::RedisError> {
//        let mut args: Vec<Vec<u8>> = Vec::new();
        let mut keys: Vec<Vec<u8>> = Vec::new();
        let code = self.statements.join("\n");
        let mut hash = Sha1::new();
        hash.update(code.as_bytes());
        let hash = hash.digest().to_string();
        self.invoke_redis(con, &hash, &code)
    }

    fn invoke_redis<T: FromRedisValue>(&self, con: &ConnectionLike,
                                       hash: &String,
                                       code: &String) -> RedisResult<T> {
        loop {
            match cmd("EVALSHA")
                .arg(hash.as_bytes())
                .arg(self.keys.len())
                .arg(&*self.keys)
                .arg(&*self.argv)
                .query(con)
                {
                    Ok(val) => {
                        return Ok(val);
                    }
                    Err(err) => if err.kind() == ErrorKind::NoScriptError {
                        let _: () = cmd("SCRIPT")
                            .arg("LOAD")
                            .arg(code.as_bytes())
                            .query(con)?;
                    } else {
                        return Err(err);
                    },
                }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_lua_script_invoke() {
        let client = redis::Client::open("redis://www.snlan.top:6379/").unwrap();
        let con = client.get_connection().unwrap();
        let mut script = super::LuaScript::new();
        script.append(r"
    return tonumber(ARGV[1]) + tonumber(ARGV[2]);
");
        script.arg(1);
        script.arg(2);

        let result = script.invoke(&con);

        match result {
            Ok(t) => {
                println!("----{}", t);
            },
            Err(e) => println!("---->{}", e),
        }
    }
}