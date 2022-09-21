extern crate serde;
extern crate toml;

#[macro_use]
extern crate failure;

use failure::Fail;
use serde_json;

#[derive(Debug, Fail)]
enum MyError {
    #[fail(display = "IO error {}.", _0)]
    Io(#[cause] std::io::Error),
    #[fail(display = "Parse error {}.", _0)]
    Parse(#[cause] std::num::ParseIntError),
    #[fail(display = "ParseSerdeJson error {}.", _0)]
    ParseSerdeJson(#[cause] serde_json::error::Error),
}

impl From<std::io::Error> for MyError {
    fn from(error: std::io::Error) -> Self {
        println!("Error: {:?}", error);
        MyError::Io(error)
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(error: std::num::ParseIntError) -> Self {
        println!("Error: {:?}", error);
        MyError::Parse(error)
    }
}

impl From<serde_json::error::Error> for MyError {
    fn from(error: serde_json::error::Error) -> Self {
        println!("Error: {:?}", error);
        MyError::ParseSerdeJson(error)
    }
}


fn parse_i32(contents: &str) -> Result<i32, MyError> {
    let n = contents.trim().parse::<i32>()?;
    Ok(2 * n)
}

fn parse_serde_json(s: &str) -> Result<serde_json::Value, MyError> {
    let value: serde_json::Value = serde_json::from_str(s)?;
    Ok(value)
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use super::*;

    #[test]
    fn it_works() {
        match parse_i32("s1") {
            Ok(n) => println!("{}", n),
            Err(err) => println!("Error: {:?}", err),
        }
        assert!(parse_i32("11").is_ok());
    }

    #[test]
    fn test_parse_serde_json() -> Result<(), MyError> {
        let s = r#"{"eagle": 18}"#;
        let res = parse_serde_json(s);
        assert!(res.is_ok());
        let s = r#"{"eagle": 18"#;
        let res = parse_serde_json(s);
        assert!(res.is_err());
        Ok(())
    }
}
