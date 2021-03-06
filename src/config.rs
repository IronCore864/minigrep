use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive;

        let c = |val: String| match val.to_lowercase().as_str() {
            "true" => true,
            "1" => true,
            _ => false,
        };

        match env::var("CASE_SENSITIVE") {
            Ok(val) => case_sensitive = c(val),
            Err(_) => case_sensitive = false,
        }

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
