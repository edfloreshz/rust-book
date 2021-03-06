use std::error::Error;
use std::{fs, env};

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool
}

impl Config {
  pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    args.next();
    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a pattern to search with.")
    };
    let filename = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a file name.")
    };
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    Ok(Config { query, filename, case_sensitive })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;
  let results = if config.case_sensitive {
    search_case_sensitive(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };
  for line in results {
    println!("{}", line);
  }
  Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents.lines()
      .filter(|line| line.contains(query))
      .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents.lines()
      .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
      .collect()
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "productive";
    assert_eq!(vec!["productive"], search_case_sensitive(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "The Rust Programming Language
Trust me";
    assert_eq!(
      vec!["The Rust Programming Language", "Trust me"],
      search_case_insensitive(query, contents)
    );
  }
}
