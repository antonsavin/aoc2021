use std::io::{BufRead, BufReader};

type Field = Vec<Vec<char>>;

fn update(field: &Field) -> Option<Field> {
    let height = field.len();
    let width = field[0].len();
    let mut moved = false;

    let mut new_field = field.clone();

    // Move right.
    for i in 0..height {
        for j in 0..width {
            if field[i][j] == '>' && field[i][(j + 1) % width] == '.' {
                new_field[i][j] = '.';
                new_field[i][(j + 1) % width] = '>';
                moved = true;
            }
        }
    }

    let mut new_field2 = new_field.clone();

    // Move down.
    for i in 0..height {
        for j in 0..width {
            if new_field[i][j] == 'v' && new_field[(i + 1) % height][j] == '.' {
                new_field2[i][j] = '.';
                new_field2[(i + 1) % height][j] = 'v';
                moved = true;
            }
        }
    }

    if moved {
        Some(new_field2)
    } else {
        None
    }
}

fn solve() {
    let reader = BufReader::new(std::io::stdin());
    let mut field = Field::new();
    let mut width: Option<usize> = None;

    for line in reader.lines() {
        let line = line.unwrap();
        let row = line.trim().chars().collect::<Vec<_>>();
        if width.is_none() {
            width = Some(row.len());
        } else {
            assert_eq!(width.unwrap(), row.len());
        }
        field.push(row);
    }

    let mut move_cnt = 0;
    loop {
        let maybe_new_field = update(&field);
        move_cnt += 1;
        match maybe_new_field {
            Some(new_field) => field = new_field,
            None => break,
        }
    }

    println!("Total moves until stop: {}", move_cnt);
}

fn main() {
    solve();
}
