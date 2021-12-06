use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
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

    let mut hori = 0;
    let mut depth = 0;


    for line in lines {
        let mut split = line.split(" ");
        let vec: Vec<&str> = split.collect();

        if vec[0] == "down" {
            depth = depth + vec[1].parse::<u32>().unwrap();
        } else if vec[0] == "up" {
            depth = depth - vec[1].parse::<u32>().unwrap();
        } else if vec[0] == "forward" {
            hori = hori + vec[1].parse::<u32>().unwrap();
        } else {
            hori = hori - vec[1].parse::<u32>().unwrap();
        }
    }
   
    println!("Product: {}", hori*depth);
}

fn part2(){
    println!("Part two:");
    
    let mut lines = lines_from_file("/src/input.txt");

    let mut aim = 0;
    let mut hori = 0;
    let mut depth = 0;


    for line in lines {
        let mut split = line.split(" ");
        let vec: Vec<&str> = split.collect();

        if vec[0] == "down" {
            aim = aim + vec[1].parse::<u32>().unwrap();
        } else if vec[0] == "up" {
            aim = aim - vec[1].parse::<u32>().unwrap();
        } else if vec[0] == "forward" {
            let mut n = vec[1].parse::<u32>().unwrap();
            hori = hori + n;
            depth = depth + aim * n
        } 
    }

    println!("Total of increases: {}", hori*depth);
}