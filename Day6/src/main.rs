use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    collections::HashMap,
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
    let mut lines = lines_from_file("/src/input.txt");

    let fish_str: Vec<&str> = lines[0].split(",").collect();
    let mut fishes: Vec<u128> = vec![0;9];
    for fish in fish_str {
        let num = fish.parse::<u128>().unwrap();
        fishes[num as usize] = fishes[num as usize] + 1;
    }
    
    println!("Total number of fishes after 64 days: {}", calculate_fish(fishes,80));
}

fn part2(){
    println!("Part two:");
    let mut lines = lines_from_file("/src/input.txt");
    
    let fish_str: Vec<&str> = lines[0].split(",").collect();
    let mut fishes: Vec<u128> = vec![0;9];
    for fish in fish_str {
        let num = fish.parse::<u128>().unwrap();
        fishes[num as usize] = fishes[num as usize] + 1;
    }
    
    println!("Total number of fishes after 256 days: {}", calculate_fish(fishes,256));
}

fn calculate_fish(mut fishes: Vec<u128>, days: u32) -> u128 {
    for i in 0..days {
        let mut new_fish: Vec<u128> = vec![0;9];
        let mut i = 0;
        while i < 9 {
            if i == 0 {
                new_fish[6] = fishes[0] + new_fish[6];
                new_fish[8] = fishes[0] + new_fish[8];
            } else {
                new_fish[i-1] = fishes[i] + new_fish[i-1];
            }
            i = i + 1;
        }
        fishes = new_fish;
    }

    let mut total_fishes = 0;
    for fish in fishes {
        total_fishes = total_fishes + fish;
    }
    return total_fishes;
}

