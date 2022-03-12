use std::env;
use std::error::Error;
use std::fs;

pub fn get_cmd_line_arg() -> Vec<String> {
    let args_list: std::vec::Vec<String> = env::args().collect();
    args_list
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    println!("Content: \n{}", content);

    let found_lines = if config.case_sensitive {
        case_sensitive_search(&config.query, &content)
    } else {
        case_insensitive_search(&config.query, &content)
    };

    println!("\nLines found:");
    for line in found_lines {
        println!("{}", line);
    }
    Ok(())
}

pub fn case_sensitive_search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut matched_lines: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            matched_lines.push(line);
        }
    }
    matched_lines
}

pub fn case_insensitive_search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut matched_lines: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            matched_lines.push(line);
        }
    }
    println!("Case Insensitive Matched Lines: {:?}", matched_lines);
    matched_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
        Rust:
safe, fast, productive.
duct cleaning
Pick three.
Duct Tape.";

        assert_eq!(
            vec!["safe, fast, productive.", "duct cleaning"],
            case_sensitive_search(query, content)
        );
    }

    #[test]
    fn case_insesitive() {
        let query = "rUsT";
        let content = "\
        Rust:
safe, fast, productive.
duct cleaning
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            case_insensitive_search(query, content)
        );
    }
}
