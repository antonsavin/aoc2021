use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let reader = BufReader::new(io::stdin());

    let mut prev_num = std::i32::MAX;
    let mut prev_num2 = std::i32::MAX;
    let mut prev_num3 = std::i32::MAX;
    let mut cnt = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let num: i32 = line.trim().parse().unwrap();
        if num > prev_num3 {
            cnt += 1;
        }

        prev_num3 = prev_num2;
        prev_num2 = prev_num;
        prev_num = num;
    }

    println!("{}", cnt);
}
