use std::error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use mysql::Error as MySqlError;
use redis::RedisError;
use serde_json::Error as JsonError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Json error {}.", _0)]
    JsonError(#[cause] JsonError),
    #[fail(display = "DB error {}.", _0)]
    DBError(#[cause] MySqlError),
    #[fail(display = "Redis error {}.", _0)]
    MemError(#[cause] RedisError),
    #[fail(display = "Common error {}.", info)]
    CommonError {
        info: String,
    },
}
