use std::env;
use std::io;

type FishCounts = [u64; 9];

fn read_nums() -> FishCounts {
    let mut first_line = String::new();
    io::stdin().read_line(&mut first_line).unwrap();

    let nums: Vec<_> = first_line
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut fish_counts: FishCounts = [0; 9];
    for num in nums {
        fish_counts[num] += 1;
    }

    fish_counts
}

fn iteration(fish_counts: FishCounts) -> FishCounts {
    let mut res: FishCounts = [0; 9];
    for i in 1..9 {
        res[i - 1] = fish_counts[i];
    }

    res[6] += fish_counts[0];
    res[8] += fish_counts[0];
    return res;
}

fn solve_part_1() {
    let mut nums = read_nums();
    for _ in 0..80 {
        nums = iteration(nums);
    }

    let sum: u64 = nums.iter().sum();
    println!("{}", sum);
}

fn solve_part_2() {
    let mut nums = read_nums();
    for _ in 0..256 {
        nums = iteration(nums);
    }

    let sum: u64 = nums.iter().sum();
    println!("{}", sum);
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
