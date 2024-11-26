/** ---------------------------------------------------------------------------------------------
 *  @Author [Tongfu.E].
 *  @Email [etongfu@outlook.com].
 *  @Date [2024-11-26 10:28:59].
 *-------------------------------------------------------------------------------------------- */
use std::error::Error;
use std::{env, fs};

pub struct Config {
  pub query: String,
  pub filename: String,
  pub ignore_case: bool
}

impl Config {
  pub fn generate(
    // args: &[String]
    mut args: impl Iterator<Item = String>
  ) -> Result<Config, &'static str> {
      args.next();

      let query = match args.next() {
          Some(arg) => arg,
          None => return Err("Didn't get a query string"),
      };

      let filename = match args.next() {
          Some(arg) => arg,
          None => return Err("Didn't get a filename string"),
      };
      

      let ignore_case = env::var("IGNORE_CASE").is_ok();

      Ok(Config { query, filename, ignore_case } )
  }
}

pub fn search<'a>(
  values: &'a str,
  query: &'a str
) -> Vec<&'a str> {
  // User for
  // let mut result = Vec::new();
  // for line in values.lines() {
  //     if line.contains(query) {
  //       result.push(line);
  //     }
  // }
  // result

  //Use filter
  values.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn case_insensitive_search<'a>(
  values: &'a str,
  query: &'a str,
) -> Vec<&'a str> {
  // let mut result = Vec::new();
  // let query = query.to_lowercase();
  // for line in values.lines() {
  //     if line.to_lowercase().contains(&query) {
  //       result.push(line);
  //     }
  // }
  // result
  let query = query.to_lowercase();
  values.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>>{

  let contents = fs::read_to_string(config.filename)?;

  let results = if config.ignore_case {
      case_insensitive_search(&contents, &config.query)
  } else {
      search(&contents, &config.query)
  };

  for line in results {
    println!("{line}")
  }

  Ok(())
}


#[cfg(test)]
mod tets{
  use std::vec;

use super::*;

  #[test]
  fn one_result() {
      let query = "safe";
      let content = "\
Rust:
safe, fast, productive.
Pick three.";
      assert_eq!(vec!["safe, fast, productive."], search(&content, &query))
  }

  #[test]
  fn case_insensitive () {
    let query = "SAFE";
      let content = "\
Rust:
safe, fast, productive.
Pick three.";
      assert_eq!(vec!["safe, fast, productive."], case_insensitive_search(&content, &query))
  }
}