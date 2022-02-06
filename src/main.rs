#[macro_use]
extern crate colour;
extern crate regex;
extern crate colored;

use std::fs;
use std::io;
use std::io::prelude::*;
use regex::Regex;
use colored::Colorize;

fn main() {
    let mut path = String::new();
    let mut query = String::new();
    green_ln!("Welcome to the string search program!");
    green!("Please enter the absolute path to the file you wish to search: ");
    std::io::stdin().read_line(&mut path).expect("Failed to read line");
    green!("Please enter the string you wish to search for: ");
    std::io::stdin().read_line(&mut query).expect("Failed to read line");
    path = path.trim().to_string();
    query = query.trim().to_string();
    if path.trim().is_empty() || query.trim().is_empty() {
        red_ln!("Filepath and/or query cannot be empty!");
        pause();
    }
    else {
        yellow_ln!("Searching for {} in {} ...\n", query, path);
        string_searcher_main(&query, &path);
        print!("\nSearch complete! ");
        pause();
    }
}

fn string_searcher_main(query: &String, file_path: &String) {
    let data = match fs::read_to_string(file_path) {
        Ok(file_data) => file_data,
        Err(e) => {
            red_ln!("Error reading file: {}", e);
            return;
        }
    }.trim().to_string();
    println!("-----------------------------------------------------");
    let mut count = 0;
    let re = Regex::new(&format!(r"({})", query.trim())[..]).unwrap();
    for (i, _) in re.captures_iter(&data).enumerate() {
        count += 1;
        cyan_ln!("Found \"{}\" in {} [{}]", query, file_path, i);
    }
    if count != 0 {
        green_ln!("Found {} occurrence(s) of \"{}\" in {}", count, query, file_path);
    }
    else {
        red_ln!("Could not find \"{}\" in {}", query, file_path);
    }
    println!("-----------------------------------------------------");
    let highlighted_data = highlight_string(&String::from(data), &query);
    green_ln!("\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("{}", highlighted_data);
    green_ln!("\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
}

fn highlight_string(data: &String, query: &String) -> String {
    let re = Regex::new(&format!(r"({})", query.trim())[..]).unwrap();
    let mut highlighted_data = String::new();
    let mut last_index = 0;
    for cap in re.captures_iter(&data) {
        highlighted_data.push_str(&data[last_index..cap.get(0).unwrap().start()]);
        highlighted_data.push_str(&format!("{}", cap.get(0).unwrap().as_str().green()));
        last_index = cap.get(0).unwrap().end();
    }
    highlighted_data.push_str(&data[last_index..]);
    highlighted_data
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    write!(stdout, "Press enter to exit ...").unwrap();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}

