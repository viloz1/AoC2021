
//Yes, I am aware that this could be refactored

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
    let vents = parse_input(lines);
    let mut field: HashMap<(i32,i32),i32> = HashMap::new();

    for vent in vents {
        if vent.start.0 == vent.end.0 {
            if vent.start.1 < vent.end.1 {
                let mut i: i32 = vent.start.1;
                while i <= vent.end.1 {
                    if field.contains_key(&(vent.start.0,i)) {
                        let cur_val = field.get(&(vent.start.0,i)).unwrap();
                        field.insert((vent.start.0,i), cur_val + 1);
                    } else {
                        field.insert((vent.start.0,i), 1);
                    }
                    i = i + 1;
                }
            } else if vent.start.1 > vent.end.1 {
                let mut i: i32 = vent.start.1;
                while i >= vent.end.1 {
                    if field.contains_key(&(vent.start.0,i)) {
                        let cur_val = field.get(&(vent.start.0,i)).unwrap();
                        field.insert((vent.start.0,i), cur_val + 1);
                    } else {
                        field.insert((vent.start.0,i), 1);
                    }
                    i = i - 1;
                }
            }
        } else if vent.start.1 == vent.end.1 {
            if vent.start.0 < vent.end.0 {
                let mut i: i32 = vent.start.0;
                while i <= vent.end.0 {
                    if field.contains_key(&(i,vent.start.1)) {
                        let cur_val = field.get(&(i,vent.start.1)).unwrap();
                        field.insert((i,vent.start.1), cur_val + 1);
                    } else {
                        field.insert((i,vent.start.1), 1);
                    }
                    i = i + 1;
                }
            } else if vent.start.0 > vent.end.0 {
                let mut i: i32 = vent.start.0;
                while i >= vent.end.0 {
                    if field.contains_key(&(i,vent.start.1)) {
                        let cur_val = field.get(&(i,vent.start.1)).unwrap();
                        field.insert((i,vent.start.1), cur_val + 1);
                    } else {
                        field.insert((i,vent.start.1), 1);
                    }
                    i = i - 1;
                }
            }

        }
    }
    
    println!("Total of increases: {}", calculate_overlaps(&mut field));
}

fn part2(){
    println!("Part two:");
    let mut lines = lines_from_file("/src/input.txt");
    let vents = parse_input(lines);
    let mut field: HashMap<(i32,i32),i32> = HashMap::new();

    for vent in vents {
        if vent.start.0 == vent.end.0 {
            if vent.start.1 < vent.end.1 {
                let mut i: i32 = vent.start.1;
                while i <= vent.end.1 {
                    if field.contains_key(&(vent.start.0,i)) {
                        let cur_val = field.get(&(vent.start.0,i)).unwrap();
                        field.insert((vent.start.0,i), cur_val + 1);
                    } else {
                        field.insert((vent.start.0,i), 1);
                    }
                    i = i + 1;
                }
            } else if vent.start.1 > vent.end.1 {
                let mut i: i32 = vent.start.1;
                while i >= vent.end.1 {
                    if field.contains_key(&(vent.start.0,i)) {
                        let cur_val = field.get(&(vent.start.0,i)).unwrap();
                        field.insert((vent.start.0,i), cur_val + 1);
                    } else {
                        field.insert((vent.start.0,i), 1);
                    }
                    if (i==0 && vent.end.0 == 0) {
                        break;
                    }
                    i = i - 1;
                }
            }
        } else if vent.start.1 == vent.end.1 {
            if vent.start.0 < vent.end.0 {
                field = calculate_path(field, vent.end.0, 1, 0, 1, vent.start.0, vent.start.1);
                
            } else if vent.start.0 > vent.end.0 {
                field = calculate_path(field, vent.end.0, -1, 0, -1, vent.start.0, vent.start.1);
            }
        } else {
            if vent.start.0 > vent.end.0 && vent.start.1 > vent.end.1 {
                field = calculate_path(field, vent.end.0, -1, -1, -1, vent.start.0, vent.start.1);
                
            } else if vent.start.0 < vent.end.0 && vent.start.1 < vent.end.1{
                field = calculate_path(field, vent.end.0, 1, 1, 1, vent.start.0, vent.start.1);
                
            } else if vent.start.0 < vent.end.0 && vent.start.1 > vent.end.1{
                field = calculate_path(field, vent.end.0, 1, -1, 1, vent.start.0, vent.start.1);
                
            } else if vent.start.0 > vent.end.0 && vent.start.1 < vent.end.1{
                field = calculate_path(field, vent.end.0, -1, 1, -1, vent.start.0, vent.start.1);
                
            }

        }
    }
    println!("Total of increases: {}", calculate_overlaps(&mut field));
}

#[derive(Debug)]
struct Vent {
    start: (i32,i32),
    end: (i32,i32),
}

fn calculate_overlaps(map: &mut HashMap<(i32,i32),i32>) -> i32 {
    let mut total_overlaps = 0;
    for (key, value) in &*map {
        if value > &1 {
            total_overlaps = total_overlaps + 1;
        }
    }
    map.clear();
    return total_overlaps;
}

fn parse_input(lines: Vec<String>) -> Vec<Vent> {
    let mut vents: Vec<Vent> = Vec::new();
    for line in lines {
        let no_space: Vec<&str> = line.split(" ").collect();
        let start_string: Vec<&str> = no_space[0].split(',').collect();

        let end_string: Vec<&str> = no_space[2].split(',').collect();
        let vent: Vent = Vent {
            start: (start_string[0].parse::<i32>().unwrap(),start_string[1].parse::<i32>().unwrap()),
            end: (end_string[0].parse::<i32>().unwrap(),end_string[1].parse::<i32>().unwrap())
        };
        vents.push(vent);
    }
    return vents;
}

fn calculate_path(mut field: HashMap<(i32,i32),i32>, end: i32, i_sign: i32, k_sign: i32, direction: i32, x_start: i32, y_start: i32) -> HashMap<(i32,i32),i32> {
    let mut i: i32 = x_start;
    let mut k: i32 = 0;
    while i * direction <= end * direction  {
        if field.contains_key(&(i,y_start+k)) {
            let cur_val = field.get(&(i,y_start+k)).unwrap();
            field.insert((i,y_start+k), cur_val + 1);
        } else {
            field.insert((i,y_start+k), 1);
        }
        i = i + i_sign;
        k = k + k_sign;
    }
    return field
}