use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::PathBuf,
    time::Instant,
};

fn build_path(filename: &str) -> PathBuf {
    // Since we're reading from the build directory, we need to do some
    // footwork to get to the right directory
    let mut cwd = PathBuf::from(&std::env::current_exe().unwrap());

    // Step back three times so that we're at the root of the project
    cwd.pop();
    cwd.pop();
    cwd.pop();

    // Then add the file name so we can reference it
    cwd.push(filename);

    cwd
}

fn get_lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(build_path(filename)).expect("Could not find file.");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| {
            l.expect("Could not read line")
        })
        .collect()
}

#[derive(Clone, Copy, Debug)]
struct BingoNumber {
    marked: bool,
    value: i64,
    row: i64,
    column: i64,
}

impl BingoNumber {
    fn set_marked(&mut self) {
        self.marked = true;
    }
}


#[derive(Clone, Debug)]
struct Board {
    board_index: i64,
    bingo_numbers: Vec<BingoNumber>,
}

fn parse_lines(lines: &Vec<String>) -> (Vec<i64>, Vec<Board>) {
    let drawn_numbers = lines.get(0).unwrap().clone().split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let mut boards: Vec<Board> = vec![];

    let mut idx: usize = 0;
    let mut board_index = 0;
    let board_to_build = &lines[2..];

    while idx < board_to_build.len() - 4 {
        let working_board_values = &board_to_build[idx..idx + 5];
        let mut working_board: Board = Board { bingo_numbers: vec![], board_index };

        for (row_idx, row) in working_board_values.into_iter().enumerate() {
            let columns_unfiltered = row.split(" ").collect::<Vec<&str>>();
            let columns = columns_unfiltered.into_iter().filter(|c| c != &"").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

            for (col_idx, value) in columns.into_iter().enumerate() {
                working_board.bingo_numbers.push(BingoNumber {
                    marked: false,
                    value,
                    row: (row_idx as i64).try_into().unwrap(),
                    column: (col_idx as i64).try_into().unwrap()
                });
            }
        }

        boards.push(working_board);
        idx += 6;
        board_index += 1;
    }

    return (drawn_numbers, boards);
}

fn mark_values(boards: &mut Vec<Board>, bingo_value: i64) -> Vec<Board> {
    let mut mutable_boards = boards.clone();
    for board in &mut mutable_boards {
        for bingo_number in &mut board.bingo_numbers {
            if bingo_number.value == bingo_value {
                bingo_number.set_marked();
            }
        }
    }

    return mutable_boards;
}

fn check_bingo(boards: &Vec<Board>) -> (bool, i64) {
    let mut has_bingo = false;
    let mut winning_board = -1;
    for (board_idx, board) in boards.into_iter().enumerate() {
        for idx in 0..4 {
            let columns = board.bingo_numbers.clone().into_iter().filter(|bn| bn.column == idx).collect::<Vec<BingoNumber>>();
            let rows = board.bingo_numbers.clone().into_iter().filter(|bn| bn.row == idx).collect::<Vec<BingoNumber>>();

            // check columns
            let mut has_column = true;
            for bingo_number in columns {
                if bingo_number.marked != true {
                    has_column = false;
                }
            }

            let mut has_row = true;
            // check rows
            for bingo_number in rows {
                if bingo_number.marked != true{
                    has_row = false;
                }
            }
            
            if has_column == true || has_row == true {
                has_bingo = true;
                winning_board = board_idx as i64;
            }
        }
    }

    return (has_bingo, winning_board);
}

fn main() {
    let lines = get_lines_from_file("input.txt");

    let now = Instant::now();

    let (drawn_numbers, mut boards) = parse_lines(&lines);

    let mut idx = 0;
    let mut has_bingo = false;
    let mut bingo_number = 0;
    let mut winning_board: Option<Board> = None;

    while !has_bingo && idx < drawn_numbers.len() {
        bingo_number = drawn_numbers.get(idx).unwrap().clone();
        boards = mark_values(&mut boards, bingo_number);

        let (found_bingo, winning_idx) = check_bingo(&boards);
        if found_bingo {
            has_bingo = true;
            winning_board = Some(boards.get(winning_idx as usize).unwrap().clone()); 
        }
        idx += 1;
    }

    let sum_unmarked: i64 = winning_board.unwrap().bingo_numbers.into_iter().filter(|bn| !bn.marked).map(|bn| bn.value).sum();

    println!("Solution: {}", sum_unmarked * bingo_number);
    println!(
        "Ran in {}ms, ({}mic)",
        now.elapsed().as_millis(),
        now.elapsed().as_micros()
    );
}
