use std::io::stdin;

enum Move {
    Up,
    Down,
    Right,
    Left,
}

fn main() {
    let integer: i32;

    loop {
        println!("Please insert positive integer number!");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        integer = match input.trim().parse() {
            Ok(num) => {
                if num <= 0 {
                    continue;
                }
                num
            }
            Err(_) => continue,
        };
        break;
    }

    let mut matrix: Vec<Vec<i32>> = vec![vec![0; integer as usize]; integer as usize];
    let moves_count = 2 * integer - 1;
    let mut step_count = 1;
    let mut number = integer * integer;
    let mut row_idx;
    let mut col_idx;
    let moves: Vec<Move>;

    if integer % 2 == 0 {
        row_idx = integer / 2;
        col_idx = integer / 2 - 1;
        moves = vec![Move::Right, Move::Up, Move::Left, Move::Down];
    } else {
        row_idx = (integer - 1) / 2;
        col_idx = row_idx;
        moves = vec![Move::Left, Move::Down, Move::Right, Move::Up];
    }
    matrix[row_idx as usize][col_idx as usize] = number;

    for n_move in 1..=moves_count {
        let current_move = &moves[(n_move - 1) as usize % 4];
        for _ in 0..step_count {
            match current_move {
                Move::Up => {
                    row_idx -= 1;
                }
                Move::Down => {
                    row_idx += 1;
                }
                Move::Left => {
                    col_idx -= 1;
                }
                Move::Right => {
                    col_idx += 1;
                }
            }

            number -= 1;
            matrix[row_idx as usize][col_idx as usize] = number;
        }

        if n_move % 2 == 0 && n_move < moves_count - 1 {
            step_count += 1;
        }
    }

    print_spiral(matrix);
}

fn print_spiral(matrix: Vec<Vec<i32>>) {
    for row in matrix {
        for cell in &row {
            print!("|{}", format!("{:width$}", cell, width = 4));
        }
        print!("|\n");
        for _ in 0..row.len() {
            print!("-----");
        }
        print!("-\n");
    }
}
