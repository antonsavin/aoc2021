use std::env;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn solve_part_1() {
    let reader = BufReader::new(io::stdin());

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut fields = line.split(' ');
        let cmd = fields.next().unwrap();
        let amount: i32 = fields.next().unwrap().parse().unwrap();
        match cmd {
            "forward" => x += amount,
            "up" => y -= amount,
            "down" => y += amount,
            _ => panic!("Unkwnown cmd {}", cmd),
        }
    }

    println!("{} {} {}", x, y, x * y);
}

fn solve_part_2() {
    let reader = BufReader::new(io::stdin());

    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut aim: i64 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut fields = line.split(' ');
        let cmd = fields.next().unwrap();
        let amount: i64 = fields.next().unwrap().parse().unwrap();
        match cmd {
            "forward" => {
                x += amount;
                y += aim * amount;
            }
            "up" => aim -= amount,
            "down" => aim += amount,
            _ => panic!("Unkwnown cmd {}", cmd),
        }
    }

    println!("Part 2: {} {} {}", x, y, x * y);
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
