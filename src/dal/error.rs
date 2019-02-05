use std::error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use mysql::Error as MySqlError;
use redis::RedisError;
use std::sync::PoisonError;
use dal::table::Table;

#[derive(Debug)]
pub enum Error {
    JsonError(String),
    DBError(MySqlError),
    MemError(RedisError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "dal error: {}", "impl")
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::JsonError(ref err) => err.as_str(),
            Error::DBError(ref err) => err.description(),
            Error::MemError(ref err) => err.description(),
        }
    }
//
//    fn cause(&self) -> Option<&error::Error> {
//        match *self {
//            Error::JsonError(ref err) => Some(err),
//            Error::DBError(ref err) => Some(err),
//            Error::MemError(ref err) => Some(err),
//        }
//    }
}

impl From<PoisonError<Table>> for Error {
    fn from(_: PoisonError<Table>) -> Self {
        Error::JsonError("poison error".to_string())
    }
}