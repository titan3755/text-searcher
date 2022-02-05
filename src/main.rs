#[macro_use]
extern crate colour;

use std::fs;
use std::io;
use std::io::prelude::*;

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
        println!("Please enter a valid path and/or query");
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
    let mut count = 0;
    println!("-----------------------------------------------------");
    for (i, x) in data.split(" ").enumerate() {
        if x.trim() == query {
            cyan_ln!("Found \"{}\" in {} [{}]", query, file_path, i);
            count += 1;
        }   
    }
    if count == 0 {
        red_ln!("Could not find \"{}\" in {}", query, file_path);
    }
    else {
        green_ln!("Found {} occurrences of \"{}\" in {}", count, query, file_path);
    }
    println!("-----------------------------------------------------");
    let highlighted_data = highlight_string(&String::from(data), &query);
    println!("{}", highlighted_data);
}

fn highlight_string(data: &String, query: &String) -> String {
    let mut highlighted_data = String::new();
    let mut index = 0;
    for x in data.split(" ") {
        if x == query {
            highlighted_data.push_str(&data[index..index + x.len()]);
            highlighted_data.push_str("\x1b[31m");
            highlighted_data.push_str(x);
            highlighted_data.push_str("\x1b[0m");
            index += x.len();
        }
        else {
            highlighted_data.push_str(x);
            highlighted_data.push_str(" ");
        }
    }
    highlighted_data
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}

