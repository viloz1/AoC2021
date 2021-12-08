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
    let string = lines.remove(0);
    let bingo_numbers: Vec<&str> = string.split(",").filter(|&x| !x.is_empty()).collect();
    lines.remove(0); 
    let mut boards: Vec<Vec<BoardNumber>> = parse_board(lines);
    for number in bingo_numbers {
        boards = update(boards, number.parse::<u32>().unwrap());
        let winning_board = check(&boards,number.parse::<u32>().unwrap());
        match winning_board {
            Some(x) => {
                println!("Number of board: {}", x);  
                break;
            },
            None => continue,
        } 
    }
    
}


fn part2(){
    println!("Part two:");
    let mut lines = lines_from_file("/src/input.txt");
    let string = lines.remove(0);
    let bingo_numbers: Vec<&str> = string.split(",").filter(|&x| !x.is_empty()).collect();
    lines.remove(0);
    let mut boards: Vec<Vec<BoardNumber>> = parse_board(lines);
    for number in bingo_numbers {
        boards = update(boards, number.parse::<u32>().unwrap());
        if (boards.len() == 1 && check_bingo(&boards[0])){
            println!("Number of last board: {}", calculate_board(&boards[0], number.parse::<u32>().unwrap()));
            break;
        } 
        let non_winning_boards = check_non_winning(boards,number.parse::<u32>().unwrap());
      
        boards = non_winning_boards;
   
    }
}

#[derive(Debug)]
struct BoardNumber {
    marked: bool,
    value: u32,
}

fn calculate_board(board: &Vec<BoardNumber>, num: u32) -> u32 {
    let mut return_num = 0;
    for number in board {
        if number.marked == false {
            return_num = return_num + number.value;
        }
    }
    return return_num*num
}

fn check(boards: &Vec<Vec<BoardNumber>>, bingo_number: u32) -> Option<u32> {
    let mut new_board: Vec<Vec<BoardNumber>> = Vec::new();
    for board in boards{
        let bingo = check_bingo(&board);
        if bingo {
            return Some(calculate_board(&board, bingo_number));
        }
    }
    return None;
}

fn check_non_winning(boards: Vec<Vec<BoardNumber>>, bingo_number: u32) -> Vec<Vec<BoardNumber>> {
    let mut new_board: Vec<Vec<BoardNumber>> = Vec::new();
    for board in boards{
        let bingo = check_bingo(&board);
        if !bingo {
            new_board.push(board);
        }
    }
    return new_board;
}

fn update(boards: Vec<Vec<BoardNumber>>, bingo_number: u32) -> Vec<Vec<BoardNumber>> {
    let mut new_boards: Vec<Vec<BoardNumber>> = Vec::new();
    for board in boards {
        let mut new_board: Vec<BoardNumber> = Vec::new();
        for i in 0..board.len() {
            if board[i].value == bingo_number || board[i].marked == true{
                new_board.push(BoardNumber{
                    value: board[i].value,
                    marked: true,
                });
            } else {
                new_board.push(BoardNumber{
                    value: board[i].value,
                    marked: false,
                });
            }

        }
        new_boards.push(new_board);
    }
    return new_boards;
}



fn parse_board(input: Vec<String>) -> Vec<Vec<BoardNumber>> {
    let mut boards: Vec<Vec<BoardNumber>> = Vec::new();
    let mut board: Vec<BoardNumber> = Vec::new();
    for line in input{
        if line == "" {
            boards.push(board);
            board = Vec::new();
        }
        let mut row = line.split(" ").filter(|&x| !x.is_empty());

        for number in row {
            let num = number.parse::<u32>().unwrap();
            let entry = BoardNumber{
                marked: false,
                value: num,
            };
            board.push(entry);
        }
    }
    boards.push(board);
    return boards
}

fn check_bingo(board: &Vec<BoardNumber>) -> bool {

    //Check horizontal bingo
    for k in (0..=4) {
        let mut bingo = true;
        for i in (0+k*5..=4+k*5) {
            if board[i].marked == false {
                bingo = false;
                break;
            }
        }

        if bingo {
            return true
        }
    }

    //Check vertical bingo
    for k in (0..=4) {
        let mut bingo = true;
        for i in (0+k..=20+k).step_by(5) {
            if board[i].marked == false {
                bingo = false;
                break;
            }
        }

        if bingo {
            return true
        }
    }
    
    return false
}