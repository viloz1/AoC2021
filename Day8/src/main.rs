
//------DISCLAIMER-------
//Don't bother to try to understand.
//I don't.
//----------------------

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    env::*,
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
    let lines = lines_from_file("/src/input.txt");
    let mut outputs: Vec<String> = Vec::new();

    for line in lines {
        let split: Vec<&str> = line.split('|').collect();
        outputs.push(split[1].to_string());
    }
    let mut total = 0;
    for output in outputs {
        let output_list: Vec<&str> = output.split(' ').filter(|&x| !x.is_empty()).collect();
        for num in output_list {
            if num.len() == 2 || num.len() == 4 || num.len() == 3 || num.len() == 7 {
                total += 1;
            }
        }
    }


    
    println!("Total apperances of 1, 4, 7 or 8: {}", total);
}

fn part2(){
    println!("Part two:");
    //Lös ett nummer i taget? Högst upp är lätt att lista ut, gör säkert så att
    //saker faller på plats

    let lines = lines_from_file("/src/input.txt");

    let mut inputs: Vec<String> = Vec::new();
    let mut outputs: Vec<String> = Vec::new();

    for line in lines {
        let split: Vec<&str> = line.split('|').collect();
        inputs.push(split[0].to_string());
        outputs.push(split[1].to_string());
    }

    let mut return_value: u128 = 0;
    let mut i = 0;

    while i < inputs.len() {
        let mut input_list: Vec<&str> = inputs[i].split(' ').filter(|&x| !x.is_empty()).collect();
        let mut output_list: Vec<&str> = outputs[i].split(' ').filter(|&x| !x.is_empty()).collect();

        let mut map: HashMap<String, char> = HashMap::new();

        map = calculate_top(&input_list, map);
        map = calculate_bottom_right(&input_list, map);
        map = calculate_top_right(&input_list, map);
        map = calculate_bottom(&input_list, map);
        map = calculate_middle(&input_list, map);
        map = calculate_top_left(&input_list, map);
        map = calculate_bottom_left(&input_list, map);


        let mut conversion: Vec<char> = iterate_over_map(&mut map);


        let mut num_string = "".to_string();

        for output in output_list {
            let mut new_order = "".to_string();
            for c in output.chars() {
                if c == conversion[0] {
                    new_order.push('a')
                } else if c == conversion[1] {
                    new_order.push('b')
                } else if c == conversion[2] {
                    new_order.push('c')
                } else if c == conversion[3] {
                    new_order.push('d')
                } else if c == conversion[4] {
                    new_order.push('e')
                } else if c == conversion[5] {
                    new_order.push('f')
                } else {
                    new_order.push('g')
                }
            }
            let mut l: Vec<char> = new_order.chars().collect();
            l.sort_unstable();
            let final_new_order: String = l.into_iter().collect();

            if final_new_order == "abcefg" {
                num_string.push('0');
            } else if final_new_order == "cf" {
                num_string.push('1');
            } else if final_new_order == "acdeg" {
                num_string.push('2');
            } else if final_new_order == "acdfg" {
                num_string.push('3');
            } else if final_new_order == "bcdf" {
                num_string.push('4');
            } else if final_new_order == "abdfg" {
                num_string.push('5');
            } else if final_new_order == "abdefg" {
                num_string.push('6');
            } else if final_new_order == "acf" {
                num_string.push('7');
            } else if final_new_order == "abcdefg" {
                num_string.push('8');
            } else {
                num_string.push('9');
            }
        }

        let num = num_string.parse::<u128>().unwrap();
    
        return_value += num;
        i += 1;

    }

    println!("Sum: {}", return_value);
}

fn iterate_over_map(map: &mut HashMap<String, char>) -> Vec<char> {
    let mut conversion: Vec<char> = vec!['a','a','a','a','a','a','a'];
    for (key, value) in &*map {
        if key == "top" {
            conversion[0] = *value;
        } else if key == "top_left" {
            conversion[1] = *value;
        } else if key == "top_right" {
            conversion[2] = *value;
        } else if key == "middle" {
            conversion[3] = *value;
        } else if key == "bottom_left" {
            conversion[4] = *value;
        } else if key == "bottom_right" {
            conversion[5] = *value;
        } else {
            conversion[6] = *value;
        } 
    }

    
    return conversion;
}

fn do_it(map: &mut HashMap<String, char>) {
    for (key, value) in &*map {
        println!("{} is {}", key, value);
    }
    map.clear();
}

fn calculate_top(inputs: &Vec<&str>, mut map: HashMap<String,char>) -> HashMap<String,char> {
    let mut two_word: String = "".to_string();
    let mut three_word: String = "".to_string();
    let mut top_char: char = 'a';

    for input in inputs {
        if input.len() == 2 {
            two_word = input.to_string();
        } else if input.len() == 3 {
            three_word = input.to_string();
        }
    }
    for c in three_word.chars() {
        if !two_word.contains(c) {
            top_char = c;
            break;
        }
    }

    map.insert(String::from("top"),top_char);
    return map;
}

fn calculate_bottom_right(inputs: &Vec<&str>, mut map: HashMap<String,char>) -> HashMap<String,char> {
    let mut number_seven = "".to_string();
    let mut number_six = "".to_string();
    let mut top_right = 'a';
    for input in inputs {
        if input.len() == 3 {
            number_seven = input.to_string();
        }
    }

    //Lets find number 6
    for input in inputs {
        if input.len() == 6 {
            let mut matches = 0;

            for c in number_seven.chars() {
                if input.contains(c) && c != *map.get("top").unwrap() {
                    matches += 1;
                    top_right = c;
                }
            }
            if matches == 1 {
                map.insert(String::from("bottom_right"),top_right);
                return map;
            }
        }
    }
    return map;
}


fn calculate_top_right(inputs: &Vec<&str>, mut map: HashMap<String,char>) -> HashMap<String,char> {
    for input in inputs {
        if input.len() == 3 {
            for c in input.chars() {
                if c != *map.get("top").unwrap() && c != *map.get("bottom_right").unwrap() {
                    map.insert(String::from("top_right"),c);
                    return map;
                }
            }
        }
    }
    return map
}

fn calculate_bottom(inputs: &Vec<&str>, mut map: HashMap<String,char>) -> HashMap<String,char> {
    //Using 4 and 9.
    let mut four = "".to_string();
    let mut nine = "".to_string();
    for input in inputs {
        if input.len() == 4 {
            four = input.to_string();
        }
    }

    for input in inputs {
        if input.len() == 6 {
            let mut matches = 0;
            for c in four.chars(){
                if input.contains(c) {
                    matches += 1;
                }
            }
            if matches == 4 {
                nine = input.to_string();
                break;
            }
        }
    }

    for c in nine.chars() {
        if !four.contains(c) && c != *map.get("top").unwrap() {
            map.insert(String::from("bottom"),c);
            return map;
        }
    }

    return map
}

fn calculate_middle(inputs: &Vec<&str>, mut map: HashMap<String,char>) -> HashMap<String,char> {
    //Using 4 and 2. Only two matches between 2 and 4, with 2 being the only of length 5 that fills
    //this criteria
    let mut four = "".to_string();
    let mut two = "".to_string();

    for input in inputs {
        if input.len() == 4 {
            four = input.to_string();
        }
    }

    for input in inputs {
        if input.len() == 5 {
            let mut matches = 0;
            for c in four.chars(){
                if input.contains(c) {
                    matches += 1;
                }
            }
            if matches == 2 {
                two = input.to_string();
                break;
            }
        }
    }

    for c in two.chars() {
        if four.contains(c) && c != *map.get("top_right").unwrap()  {
            map.insert(String::from("middle"),c);
            return map;
        }
    }

    return map;
}

fn calculate_top_left(inputs: &Vec<&str>, mut map: HashMap<String,char>) -> HashMap<String,char> {
    //At this point, we know every position in nine except for top-left
    let mut nine = "".to_string();
    for input in inputs {
        if input.len() == 6 {
            if input.contains(*map.get("middle").unwrap()) && input.contains(*map.get("top_right").unwrap()) {
                for c in input.chars() {
                    if (c != *map.get("middle").unwrap() && c != *map.get("top").unwrap() && c != *map.get("top_right").unwrap() &&
                        c != *map.get("bottom_right").unwrap() && c != *map.get("bottom").unwrap()) {
                            map.insert(String::from("top_left"),c);
                            return map;
                        }
                }
            }
        }
    }

    return map;
}

fn calculate_bottom_left(inputs: &Vec<&str>, mut map: HashMap<String,char>) -> HashMap<String,char> {
    //Only one left
    let list = "abcdefg".to_string();
    for c in list.chars() {
        if (c != *map.get("middle").unwrap() && c != *map.get("top").unwrap() && c != *map.get("top_right").unwrap() &&
            c != *map.get("bottom_right").unwrap() && c != *map.get("bottom").unwrap() && c != *map.get("top_left").unwrap()) {
                map.insert(String::from("bottom_left"),c);
                return map;
        }
    }
    return map
}