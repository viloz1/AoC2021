use relative_path::RelativePath;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    env::*,
};

fn lines_from_file(filename: &str) -> Vec<String> {
    let paths = std::env::current_dir().unwrap();
    let full_path = format!("{}{}", &paths.into_os_string().into_string().unwrap(), &filename);
    let file = File::open(Path::new(&full_path)).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    part1();
    part2();
}

fn part1() {
    println!("Part one:");
    let lines = lines_from_file("/src/input.txt");
    let mut previous = 0;
    let mut total = -1;

    for row in lines {
        let number = row.parse().unwrap();
        if (previous < number) {
            total = total + 1;
        }
        previous = number;
    }
    println!("Total of increases: {}", total);
}

fn part2(){
    println!("Part two:");
    let lines = lines_from_file("/src/input.txt");
    let mut new_vec = Vec::new();
    let mut sum_number: u32 = 0;
    let length = lines.len();
    let mut i = 0;

    while i < length -2 {
        sum_number = lines[i].parse::<u32>().unwrap() + lines[i+1].parse::<u32>().unwrap() + lines[i+2].parse::<u32>().unwrap();
        new_vec.push(sum_number);
        sum_number = 0;
        i = i + 1;
    }

    let mut previous = 0;
    let mut total = -1;

    for entry in new_vec {
        if (previous < entry) {
            total = total + 1;
        }
        previous = entry;
    }

    println!("Total of increases: {}", total);
}