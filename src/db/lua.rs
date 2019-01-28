extern crate redis;

use self::redis::{Script, Connection, ToRedisArgs, ScriptInvocation};

pub struct LuaScript<T:ToRedisArgs> {
    statements: Vec<String>,
    keys: Vec<String>,
    key_index: i32,
    argv: Vec<T>,
    arg_index: i32,
}

impl <T:ToRedisArgs>LuaScript<T> {
    pub fn new() ->LuaScript<T> {
        LuaScript {
            statements:vec!["local total = 0".to_string(), "local r = 0".to_string()],
            keys:Vec::new(),
            key_index:1,
            argv:Vec::new(),
            arg_index:1,
        }
    }

    pub fn append(&mut self,code :&str) {
        self.statements.push(code.to_string())
    }

    pub fn arg(&mut self, arg:T) {
        self.argv.push(arg);
    }
    pub fn invoke(self, con: &Connection) -> Result<isize, redis::RedisError> {
        let script = Script::new(self.statements.join("\n").as_str());

        let mut invk = script;
        for arg in self.argv {
            let mut invk = invk.arg(arg);
        }
        invk.invoke(con)
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

        let result: isize = script.invoke(&con).unwrap();
        assert_eq!(result, 3)
    }
}