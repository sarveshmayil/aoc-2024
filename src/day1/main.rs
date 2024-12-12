use std::fs;
use std::io::{self, BufRead, BufReader};

/*
Function to read two lists from text file.
Returns two lists of integers.
*/
fn read_input(file_path: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    println!("Reading input from file: {}", file_path);

    let data = fs::read_to_string(filepath).expect("Unable to read {}", file_path);

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in data.lines() {
        let mut split = line.split(" ");
        let mut list1 = split.next().unwrap().parse::<i32>().unwrap();
        let mut list2 = split.next().unwrap().parse::<i32>().unwrap();
    }
}

fn main() {
    let input = read_input("input.txt").expect("Failed to read input.txt");
    println!("{:?}", input);
}