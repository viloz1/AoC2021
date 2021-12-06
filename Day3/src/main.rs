
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
    let mut vec: Vec<(u32,u32)> = vec![(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0)];
    for line in lines {
        for (i, char) in line.chars().enumerate(){
            if char == '0' {
                vec[i].0 = vec[i].0 + 1;
            } else {
                vec[i].1 = vec[i].1 + 1;
            }
        }
    }

    let mut gamma_rate_bin: String = String::from("");
    let mut epsilon_rate_bin: String = String::from("");

    for entry in vec {
        if entry.0 > entry.1 {
            gamma_rate_bin.push('0');
            epsilon_rate_bin.push('1');
        } else {
            gamma_rate_bin.push('1');
            epsilon_rate_bin.push('0');
        }
    }

    let gamma_rate_bin_slice: &str = &gamma_rate_bin[..];
    let epsilon_rate_bin_slice: &str = &epsilon_rate_bin[..];

    let gamma_rate = isize::from_str_radix(gamma_rate_bin_slice, 2).unwrap();
    let epsilon_rate = isize::from_str_radix(epsilon_rate_bin_slice, 2).unwrap();

    println!("Product: {}", gamma_rate*epsilon_rate);
}

fn part2(){
    println!("Part two:");

    let mut oxygen_generator_rating_bin = get_rating('0','1');
    let mut co2_scrubber_rating_bin = get_rating('1','0');
    
    let oxygen_generator_rating_slice: &str = &oxygen_generator_rating_bin[..];
    let co2_scrubber_rating_slice: &str = &co2_scrubber_rating_bin[..];

    let oxygen_generator_rating = isize::from_str_radix(oxygen_generator_rating_slice, 2).unwrap();
    let co2_scrubber_rating = isize::from_str_radix(co2_scrubber_rating_slice, 2).unwrap();

    println!("Life support rating: {}", co2_scrubber_rating * oxygen_generator_rating);
}

fn get_rating(symbol: char, opposite_symbol: char) -> String {
    let mut i = 0;
    let mut co2_scrubber_rating_bin = String::from("");

    let mut candidates: Vec<String> = Vec::new();
    candidates =  lines_from_file("/src/input.txt");
    let length = candidates[0].chars().count();
    while i < length {

        let mut control_bit = '0';
        let mut apperances = (0,0);
        for line in &mut candidates {
            let chars: Vec<char> = line.chars().collect();
            if chars[i] == '0' {
                apperances.0 = apperances.0 + 1;
            } else {
                apperances.1 = apperances.1 + 1;
            }
        
        }

        if apperances.0 > apperances.1 {
            control_bit = symbol;
        } else {
            control_bit = opposite_symbol;
        }

        let mut new_candidates: Vec<String> = Vec::new();
        
        for (k, line) in candidates.iter().enumerate(){
            let c = line.chars().nth(i).unwrap();
            if c == control_bit {
                new_candidates.push(line.to_string());
            }
        }

        candidates = new_candidates;
        i = i+1;

        if candidates.len() == 1 {
            return candidates[0].clone()
        }
    }
    return String::from("")
}

