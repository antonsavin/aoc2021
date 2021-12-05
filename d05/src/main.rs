use std::collections::HashMap;
use std::env;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use text_io::scan;

fn sign(x: i32) -> i32 {
    if x < 0 {
        -1
    } else if x > 0 {
        1
    } else {
        0
    }
}

fn solve(allow_diag: bool) {
    let reader = BufReader::new(io::stdin());
    let mut counts = HashMap::<(i32, i32), i32>::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let (x1, y1, x2, y2): (i32, i32, i32, i32);
        scan!(line.bytes() => "{},{} -> {},{}", x1, y1, x2, y2);
        let dx = sign(x2 - x1);
        let dy = sign(y2 - y1);
        if dx != 0 && dy != 0 && !allow_diag {
            continue;
        }

        let (mut x, mut y) = (x1, y1);
        loop {
            *counts.entry((x, y)).or_insert(0) += 1;
            if (x, y) == (x2, y2) {
                break;
            }
            x += dx;
            y += dy;
        }
    }

    let res = counts.values().filter(|x| **x > 1).count();
    println!("Total overlap: {}", res);
}

fn solve_part_1() {
    solve(false);
}

fn solve_part_2() {
    solve(true);
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
