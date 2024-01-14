#![allow(
    clippy::all,
    missing_debug_implementations,
)]

use clap::{ Arg, Command };
use serde_json::Value;

const NAME: &str = "bs";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const JSON_ARG: &str = "json";
const TARGET_ARG: &str = "target";

fn cli() -> Command {
    Command::new(NAME)
        .version(VERSION)
        .about("Performs binary search on list of integers specified in a JSON file.")
        .args([
            Arg::new(JSON_ARG)
                .short('j')
                .help("JSON file containing list to search")
                .required(true),
            Arg::new(TARGET_ARG)
                .short('t')
                .help("Unsigned number to search for in list")
                .required(true),
        ])
}

fn get_args() -> (Vec<u64>, u64) {
    let mut matches = cli().get_matches();

    let file_path = matches.get_one::<String>(JSON_ARG).expect("Expected json file argument");
    let data = std::fs::read_to_string(file_path).expect("Unable to read file");
    let num_list: Value = serde_json::from_str(&data).expect("Could not parse JSON");
    let num_list = num_list.get("num_list").expect("\"num_list\" key does not exist")
        .as_array().expect("Could not parse array")
        .iter().map(|n| n.as_u64().unwrap())
        .collect::<Vec<u64>>();

    let target = matches.get_one::<String>(TARGET_ARG).expect("Expected target argument");
    let target = target.parse::<u64>().expect("Error parsing target value");

    ( num_list, target )
}

fn binary_search(list: Vec<u64>, target: u64) -> Option<usize> {
    let mut left: usize = 0;
    let mut right: usize = list.len() - 1;
    while left <= right {
        let mid = (left + right) / 2;
        if list[mid] == target {
            return Some(mid);
        }

        if list[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    None
}

fn main() -> () {
    let (list, target) = get_args();
    let now = std::time::Instant::now();
    let result = binary_search(list, target);
    let elapsed = now.elapsed();
    match result {
        Some(i) => println!("Found {target} at index {i}."),
        None => println!("Could not find {target} in list.")
    }

    println!("Search time: {elapsed:?}");
}
