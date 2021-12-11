use std::env;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

const SIZE: usize = 10;
const SIZE_I: i32 = SIZE as i32;

type Field = [[u8; SIZE]; SIZE];

fn new_field() -> Field {
    [[0; SIZE]; SIZE]
}

fn flash(field: &mut Field, i: i32, j: i32) {
    for i1 in i - 1..=i + 1 {
        for j1 in j - 1..=j + 1 {
            if i1 >= 0 && i1 < SIZE_I && j1 >= 0 && j1 < SIZE_I && (i1 != i || j1 != j) {
                field[i1 as usize][j1 as usize] += 1;
            }
        }
    }
}

fn read_field() -> Field {
    let reader = BufReader::new(io::stdin());
    let mut field = new_field();

    for (i, line) in reader.lines().take(SIZE).enumerate() {
        for (j, c) in line.unwrap().trim().chars().enumerate() {
            field[i][j] = c as u8 - '0' as u8;
        }
    }

    // println!("Field:\n{:?}", field);
    field
}

fn iteration(field: &mut Field) -> usize {
    let mut flashed_cnt = 0;
    let mut flashed = new_field();
    field
        .iter_mut()
        .for_each(|m| m.iter_mut().for_each(|m| *m += 1));
    let mut some_flashed = true;
    while some_flashed {
        some_flashed = false;
        for i in 0..SIZE {
            for j in 0..SIZE {
                if field[i][j] > 9 && flashed[i][j] == 0 {
                    flash(field, i as i32, j as i32);
                    flashed[i][j] = 1;
                    flashed_cnt += 1;
                    some_flashed = true;
                }
            }
        }
    }

    for i in 0..SIZE {
        for j in 0..SIZE {
            if field[i][j] > 9 {
                field[i][j] = 0;
            }
        }
    }

    flashed_cnt
}

fn solve_part_1() {
    let mut field = read_field();

    let mut total_flashed = 0;

    for _step in 1..=100 {
        total_flashed += iteration(&mut field);

        // println!("After step {}:\n{:?}", _step, field);
    }

    println!("Total flashed: {}", total_flashed);
}

fn solve_part_2() {
    let mut field = read_field();

    for step in 1.. {
        if iteration(&mut field) == SIZE * SIZE {
            println!("First step with full flash: {}", step);
            break;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_part_1 = args[1] == "1";
    if is_part_1 {
        solve_part_1();
    } else {
        solve_part_2();
    }
}
