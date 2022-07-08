use std::{env, fs::read_to_string, time::Instant};

pub fn get_file_contents(query: &str, filename: &str) {
    let contents = read_to_string(String::from(filename)).unwrap();
    let results = search(&query, &contents);
    let count = results.len();
    println!("matches: {:#?}", count);
}

pub fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn main() {
    let start = Instant::now();
    let mut args = env::args();
    args.next();
    let query = args.next().unwrap();
    let file = args.next().unwrap();
    //
    get_file_contents(&query, &file);
    let duration = start.elapsed();
    println!("operation complete in {:#?}", duration);
}
