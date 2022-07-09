use memchr::memmem::{self, Finder};
use std::{env::{self, Args}, fs::read_to_string, time::{Instant, Duration}};

pub fn get_file_contents(query: &str, filename: &str) {
    let contents: String = read_to_string(String::from(filename)).unwrap();
    let results: Vec<&str> = search(&query, &contents);
    println!("matches: {:#?}", results.len());
}

pub fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let needle: Finder = memmem::Finder::new(query);
    let mut results: Vec<&str> = vec![];
    for line in content.lines() {
        if let Some(_) = needle.find(line.as_bytes()) {
            results.push(line)
        }
    }
    results
}

fn main() {
    let start: Instant = Instant::now();
    let mut args: Args = env::args();
    args.next();
    let query: String = args.next().unwrap();
    let file: String = args.next().unwrap();
    //
    get_file_contents(&query, &file);
    let duration: Duration = start.elapsed();
    println!("operation complete in {:#?}", duration);
}
