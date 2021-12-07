use std::env;
use std::io;

fn read_nums() -> Vec<i32> {
    let mut first_line = String::new();
    io::stdin().read_line(&mut first_line).unwrap();

    let nums: Vec<_> = first_line
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    nums
}

fn solve_part_1() {
    let mut nums = read_nums();
    nums.sort();
    let median = nums[nums.len() / 2];
    let res: i32 = nums.iter().map(|x| (x - median).abs()).sum();
    println!("Min moves: {}", res);
}

fn dist_2(pos: &i32, point: f64) -> f64 {
    return (*pos as f64 - point).abs() * ((*pos as f64 - point).abs() + 1.0) / 2.0;
}

fn solve_part_2() {
    let nums = read_nums();
    let avg: f64 = nums.iter().sum::<i32>() as f64 / nums.len() as f64;
    let avg_floor = avg.floor();
    let avg_ceil = avg.ceil();
    let cost_floor: f64 = nums.iter().map(|x| dist_2(x, avg_floor)).sum();
    let cost_ceil: f64 = nums.iter().map(|x| dist_2(x, avg_ceil)).sum();
    println!(
        "Avg: {}, cost 1: {}, cost 2: {}, min: {}",
        avg,
        cost_floor,
        cost_ceil,
        cost_floor.min(cost_ceil)
    );
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
