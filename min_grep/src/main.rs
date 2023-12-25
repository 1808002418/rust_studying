use std::error::Error;

use std::process;
fn main() {
    // run --package min_grep --bin min_grep -- std E:\code\rust_study\min_grep\src\macros
    let args: Vec<String> = std::env::args().skip(1).collect();
    let config = Config::build(args.into_iter()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    if let Err(err) = run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {
        let query = args.next().expect("Didn't get a query string");
        let file_path = args.next().expect("Didn't get a file path");
        return Ok(Config::new(query, file_path));
    }

    fn new(query: String, file_path: String) -> Config {
        let flag = std::env::var("IGNORE_CASE").map_or(false, |val| val.eq("1"));
        Config { query, file_path, ignore_case: flag }
    }
}

pub fn run(mut config: Config) -> Result<(), Box<dyn Error>> {
    let mut content = std::fs::read_to_string(config.file_path)?;
    convert_lowercase(config.ignore_case, &mut content, &mut config.query);
    let mut line_num = 0;
    for line in content.lines() {
        line_num += 1;
        if query_match(&config.query, line) {
            println!("{:<4} {}", line_num, line.trim());
        }
    }
    return Ok(());
}

fn query_match(query: &str, line: &str) -> bool {
    return line.contains(query);
}

fn convert_lowercase(ignore: bool, content: &mut String, query: &mut String) {
    if ignore {
        content.make_ascii_lowercase();
        query.make_ascii_lowercase();
    }
}


