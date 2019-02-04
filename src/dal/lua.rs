extern crate redis;
extern crate sha1;

use self::redis::{Connection, ToRedisArgs, cmd};
use self::redis::FromRedisValue;
use self::redis::ConnectionLike;
use self::redis::RedisResult;
use self::redis::ErrorKind;
use self::sha1::Sha1;
use std::collections::HashMap;

const LUA_RET: &str = "total";

pub struct LuaScript {
    statements: Vec<String>,
    keys: Vec<String>,
    key_index: i32,
    argv: Vec<Vec<u8>>,
    arg_index: i32,

}

impl LuaScript {
    pub fn new() -> LuaScript {
        LuaScript {
            statements: vec!["local total = 0".to_string(), "local r = 0".to_string()],
            keys: Vec::new(),
            key_index: 0,
            argv: Vec::new(),
            arg_index: 0,
        }
    }

    fn push_arg<T: ToRedisArgs>(&mut self, arg: T) {
        arg.write_redis_args(&mut self.argv);
        self.arg_index += 1;
    }

    fn push_key(&mut self, key: String) {
        self.keys.push(key);
        self.key_index += 1;
    }

    pub fn sadd<T: ToRedisArgs + Clone>(&mut self, key: String, member: Vec<T>) {
        let mut args: Vec<String> = Vec::with_capacity(member.len());
        for v in member.into_iter() {
            self.push_arg(v);
            args.push(format!("ARGV[{}]", self.arg_index));
        }
        self.push_key(key);
        let code = format!("{} = {} + redis.call('SADD', KEYS[{}], {})\n", LUA_RET, LUA_RET, self.key_index, args.join(", "));
        self.statements.push(code);
    }

    pub fn hmset<T: ToRedisArgs + Clone>(&mut self, key: String, fv: HashMap<String, T>) {
        let fv_list = fv.into_iter()
            .map(|(f, v)| {
                self.push_key(f);
                self.push_arg(v);
                format!("KEYS[{}], ARGV[{}]", self.key_index, self.arg_index)
            })
            .collect::<Vec<_>>();
        let code = format!("r = redis.call('HMSET', '{}', {})\n", key, fv_list.join(", "));
        self.statements.push(code);
        self.statements.push(format!("if r.ok == 'OK'\nthen\n{} = {} + 1\nend\n", LUA_RET, LUA_RET));
    }

    pub fn hset<T: ToRedisArgs + Clone>(&mut self, key: String, field: String, value: T) {
        let code = format!("r = redis.call('HSET', KEYS[{}], KEYS[{}], ARGV[{}])", self.key_index + 1, self.key_index + 2, self.arg_index + 1);
        self.push_key(key);
        self.push_key(field);
        self.push_arg(value);
        self.statements.push(code);
        self.statements.push(format!("if r == 1\nthen\n{} = {} + 1\nend\n", LUA_RET, LUA_RET));
    }

    pub fn del(&mut self, keys: Vec<String>) {
        let key_list = keys.into_iter()
            .map(|key| {
                self.push_key(key);
                format!("KEYS[{}]", self.key_index)
            })
            .collect::<Vec<_>>();
        let code = format!("{} = {} + redis.call('DEL', {})\n", LUA_RET, LUA_RET, key_list.join(", "));
        self.statements.push(code);
    }

    pub fn srem(&mut self, key: String, member: Vec<String>) {
        let m_list = member.into_iter()
            .map(|k| {
                self.push_arg(k);
                format!("ARGV[{}]", self.arg_index)
            })
            .collect::<Vec<_>>();
        self.push_key(key);
        let code = format!("redis.call('SREM', KEYS[{}], {})\n", self.key_index, m_list.join(", "));
        self.statements.push(code);
    }

    pub fn expire(&mut self, key :String, seconds:u64) {
        self.push_key(key);
        let code = format!("redis.call('EXPIRE', KEYS[{}], {})\n", self.key_index, seconds);
        self.statements.push(code);
    }

    pub fn invoke(&mut self, con: &Connection) -> Result<isize, redis::RedisError> {
        self.statements.push(format!("return {}", LUA_RET));
        let code = self.statements.join("\n");
        println!("\n{}\n", code);
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
    use super::redis::Connection;
    use std::collections::HashMap;
    use dal::lua::redis::Commands;

    fn get_conn() -> Connection {
        let client = redis::Client::open("redis://:snlan@www.snlan.top:6379/").unwrap();
        client.get_connection().unwrap()
    }

    #[test]
    fn test_lua_script_sadd() {
        let con = get_conn();
        let mut script = super::LuaScript::new();
        script.sadd("name".to_string(), vec!["lucy", "alias", "bob"]);

        let r1 = script.invoke(&con).unwrap();

        assert_eq!(r1, 3);

        let mut script = super::LuaScript::new();
        script.sadd("name".to_string(), vec!["lucy", "lion"]);
        let r2 = script.invoke(&con).unwrap();
        assert_eq!(r2, 1);
    }

    #[test]
    fn test_lua_script_hmset() {
        let con = get_conn();
        let mut script = super::LuaScript::new();

        let mut fv = HashMap::new();
        fv.insert("google".to_string(), "www.google.com");
        fv.insert("yahoo".to_string(), "www.yahoo.com");

        script.hmset("website".to_string(), fv);

        let r1 = script.invoke(&con).unwrap();
        assert_eq!(r1, 1);
    }

    #[test]
    fn test_lua_script_hset() {
        let con = get_conn();
        let mut script = super::LuaScript::new();

        script.hset("website".to_string(), "name".to_string(), 12);

        let r1 = script.invoke(&con).unwrap();
        assert_eq!(r1, 1);
    }

    #[test]
    fn test_lua_script_del() {
        let con = get_conn();
        let _: () = con.set("k1", 42).unwrap();
        let _: () = con.set("k2", "hello").unwrap();
        let mut script = super::LuaScript::new();

        script.del(vec!["k1".to_string(), "k2".to_string()]);

        let r1 = script.invoke(&con).unwrap();
        assert_eq!(r1, 2);
    }

    #[test]
    fn test_lua_script_srem() {
        let con = get_conn();
        let mut script = super::LuaScript::new();

        script.srem("name".to_string(), vec!["lucy".to_string(), "alias".to_string(), "other".to_string()]);

        let r1 = script.invoke(&con).unwrap();
        assert_eq!(r1, 0);
    }

    #[test]
    fn test_lua_script_expire() {
        let con = get_conn();
        let _: () = con.set("k1", 42).unwrap();
        let mut script = super::LuaScript::new();
        script.expire("k1".to_string(), 30);
        let r1 = script.invoke(&con).unwrap();
        assert_eq!(r1, 0);
    }
}