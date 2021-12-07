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
    let positions: Vec<u128> = parse_input(lines);

    let med = positions[positions.len()/2];
    let mut total_fuel = 0;
    for num in positions {
        let value: i128 = (num as i128) - (med as i128);
        total_fuel = total_fuel + (value.abs() as u128);
    }
    println!("Total fuel: {}", total_fuel);
}

fn part2(){
    println!("Part two:");
    let lines = lines_from_file("/src/input.txt");
    let positions: Vec<u128> = parse_input(lines);

    let sum: u128 = positions.iter().sum();
    let mut med = ((sum as usize) /positions.len()) as u128;
    let mut total_fuel: u128 = 0;
    for num in positions {
        if num > med {
            total_fuel = total_fuel + calculate_sum(num,med);
        } else {
            total_fuel = total_fuel + calculate_sum(med,num);
        }
    }
    println!("Total fuel: {}", total_fuel);
}

fn parse_input(lines: Vec<String>) -> Vec<u128> {
    let lines = lines_from_file("/src/input.txt");
    let mut positions = Vec::new();
    let positions_string: Vec<&str> = lines[0].split(',').collect();
    for num_str in positions_string {
        positions.push(num_str.parse::<u128>().unwrap());
    }
    positions.sort();
    return positions
}

fn calculate_sum(start: u128, end: u128) -> u128 {
    let iterations = start-end;
    let mut count = 0;
    for i in 1..=iterations {
        count = count + i;
    }
    return count;
}