//! A commandline tool that greps a string in a file.
//! Case-sensitive search can be enabled by setting ENV var CASE_SENSITIVE=true; defaults to false.
//! # Usage:
//! ```bash
//! minigrep string filename
//! ```

use std::error::Error;
use std::fs;

pub mod config;
use config::Config;

mod search;
use search::{search_case_insensitive, search_case_sensitive};

#[cfg(test)]
mod tests;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = match config.case_sensitive {
        true => search_case_sensitive(&config.query, &contents),
        false => search_case_insensitive(&config.query, &contents),
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
