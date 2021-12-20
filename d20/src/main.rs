use std::env;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

type Field = Vec<Vec<i32>>;

fn pad(f: Field, padding: usize) -> Field {
    let m = f[0].len();
    let mut res = Field::new();
    let row_zeros = std::iter::repeat(0).take(m + 2 * padding).collect::<Vec<_>>();
    for _ in 0..padding {
        res.push(row_zeros.clone());
    }

    let pad_zeros = std::iter::repeat(0).take(padding).collect::<Vec<_>>();

    for row in f {
        let mut new_row = pad_zeros.clone();
        new_row.extend(&row);
        new_row.extend(&pad_zeros);
        res.push(new_row);
    }

    for _ in 0..padding {
        res.push(row_zeros.clone());
    }

    res
}

fn get(field: &Field, i: i32, j: i32, outside_value: i32) -> i32 {
    if i < 0 || i >= field.len() as i32 || j < 0 {
        return outside_value;
    }

    let row = &field[i as usize];
    if j >= row.len() as i32 {
        return outside_value;
    }

    return *row.get(j as usize).unwrap();
}

fn transform(field: &Field, outside_color: i32, encoding: &Vec<i32>) -> (Field, i32) {
    let mut res = field.clone();
    let n = field.len() as i32;
    let m = field.get(0).unwrap().len() as i32;

    for i in 0..n {
        for j in 0..m {
            let mut num = 0;
            for i1 in i - 1..=i + 1 {
                for j1 in j - 1..=j + 1 {
                    num = num * 2 + get(field, i1, j1, outside_color)
                }
            }
            let encoded = encoding.get(num as usize).unwrap();
            let _ = std::mem::replace(&mut res[i as usize][j as usize], *encoded);
        }
    }

    let new_outside_color = encoding[if outside_color == 0 { 0 } else { 511 }];
    (res, new_outside_color)
}

fn solve(part_1: bool) {
    let mut reader = BufReader::new(io::stdin());
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let encoding = line
        .trim()
        .chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect::<Vec<_>>();
    assert_eq!(encoding.len(), 512);

    reader.read_line(&mut line).unwrap();
    let mut field = Field::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let row = line
            .trim()
            .chars()
            .map(|c| if c == '#' { 1 } else { 0 })
            .collect::<Vec<_>>();
        field.push(row);
    }

    let iter_cnt = if part_1 { 2 } else { 50 };

    let mut field = pad(field, iter_cnt + 1);

    let mut outside_color = 0;

    for _ in 0..iter_cnt {
        let (new_field, new_outside_color) = transform(&field, outside_color, &encoding);
        field = new_field;
        outside_color = new_outside_color;
    }

    println!("Final field:");
    for row in &field {
        let row_str = row.iter().map(|x| if *x == 1 { '#' } else { '.' }).collect::<String>();
        println!("{}", row_str);
    }
    println!("Final outside color: {}", outside_color);

    let num_ones: i32 = field.iter().map(|r| r.iter().sum::<i32>()).sum();
    println!("Num ones: {}", num_ones);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_part_1 = args[1] == "1";
    if is_part_1 {
        solve(true);
    } else {
        solve(false);
    }
}
