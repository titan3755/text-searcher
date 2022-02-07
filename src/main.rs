#[macro_use]
extern crate colour;
extern crate regex;
extern crate colored;
extern crate term_table;
extern crate figlet_rs;

use std::fs;
use regex::Regex;
use colored::Colorize;
use figlet_rs::FIGfont;
use term_table::{Table, row::Row, TableStyle, table_cell::{Alignment, TableCell}};

fn main() {
    loop {
        let mut path = String::new();
        let mut query = String::new();
        let standard_font = FIGfont::standand().unwrap();
        let figure = standard_font.convert("TEXT - SEARCHER").unwrap();
        blue_ln!("{}", figure);
        green!("Please enter the absolute path to the file you wish to search: ");
        std::io::stdin().read_line(&mut path).expect("Failed to read line");
        green!("Please enter the string you wish to search for: ");
        std::io::stdin().read_line(&mut query).expect("Failed to read line");
        path = path.trim().to_string();
        query = query.trim().to_string();
        if path.trim().is_empty() || query.trim().is_empty() {
            red_ln!("Filepath and/or query cannot be empty!");
            pause("\nRetry? (y/n): ");
        }
        else {
            yellow_ln!("Searching for {} in {} ...\n", query, path);
            string_searcher_main(&query, &path);
            pause("\nSearch again? (y/n): ");
        }
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
    table_creator("Search Start");
    let mut count = 0;
    let re = Regex::new(&format!(r"({})", query.trim())[..]).unwrap();
    for (i, _) in re.captures_iter(&data).enumerate() {
        count += 1;
        cyan_ln!("Found \"{}\" in {} [{}]", query, file_path, i);
    }
    if count != 0 {
        green_ln!("Found {} occurrence(s) of \"{}\" in {}\n", count, query, file_path);
    }
    else {
        red_ln!("Could not find \"{}\" in {}", query, file_path);
    }
    table_creator("Search Completed");
    let highlighted_data = highlight_string(&String::from(data), &query);
    table_creator("Highlighted Search Results");
    println!("{}", highlighted_data);
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

fn table_creator(text: &str) {
    let mut table = Table::new();
    table.max_column_width = 100;
    table.style = TableStyle::extended(); 
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment(text, 2, Alignment::Center)
    ]));
    println!("{}", table.render());
}

fn pause(phrase: &str) {
    let mut input = String::new();
    green!(phrase);
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    if input.trim().to_lowercase() == "y" {
        return;
    }
    else {
        red_ln!("Exiting program...");
        std::process::exit(0);
    }
}

