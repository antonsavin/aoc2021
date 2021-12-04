use std::collections::HashMap;
use std::env;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn solve_part_1() {
    let reader = BufReader::new(io::stdin());

    let mut line_cnt = 0;
    let mut one_counts = HashMap::new();
    let mut first = true;
    let mut line_len = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if first {
            first = false;
            line_len = line.len();
        }

        line_cnt += 1;
        for (i, c) in line.chars().enumerate() {
            let cnt = one_counts.entry(i).or_insert(0);
            if c == '1' {
                *cnt += 1;
            }
        }
    }

    let mut gamma: u32 = 0;
    for i in 0..line_len {
        let one_cnt = one_counts.get(&i).unwrap();
        if one_cnt * 2 == line_cnt {
            panic!("Can't decide");
        }

        let bit = if one_cnt * 2 > line_cnt { 1 } else { 0 };
        gamma = (gamma << 1) + bit;
    }

    let epsilon = !gamma & !(!0 << line_len);
    println!("{} {} {}", gamma, epsilon, gamma * epsilon);
}

fn count_bits(nums: &Vec<Vec<char>>, bit_idx: usize) -> (usize, usize) {
    let cnt_zero = nums.iter().filter(|v| v[bit_idx] == '0').count();
    (cnt_zero, nums.len() - cnt_zero)
}

fn filter_by(
    name: &str,
    all_nums: &Vec<Vec<char>>,
    get_bit_fn: impl Fn(&Vec<Vec<char>>, usize) -> char,
) -> usize {
    let bit_cnt = all_nums[0].len();
    let mut cur_nums = all_nums.to_vec();

    let mut new_nums: Vec<Vec<char>> = vec![];

    for i in 0..bit_cnt {
        let bit_value = get_bit_fn(&cur_nums, i);
        for v in cur_nums.drain(..) {
            if v[i] == bit_value {
                new_nums.push(v)
            }
        }

        cur_nums.append(&mut new_nums);
    }

    let s: String = cur_nums[0].iter().collect();
    let value = usize::from_str_radix(&s, 2).unwrap();
    println!("{} {} {} {}", name, cur_nums.len(), s, value);
    value
}

fn solve_part_2() {
    let reader = BufReader::new(io::stdin());

    let mut all_nums: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();

        all_nums.push(line.chars().collect());
    }

    let most_common_bit_closure = |cur_nums: &Vec<Vec<char>>, i| {
        let (_, one_cnt) = count_bits(&cur_nums, i);
        if one_cnt * 2 >= cur_nums.len() {
            '1'
        } else {
            '0'
        }
    };

    let least_common_bit_closure = |cur_nums: &Vec<Vec<char>>, i| {
        let (zero_cnt, one_cnt) = count_bits(&cur_nums, i);
        if zero_cnt > 0 && zero_cnt <= one_cnt || one_cnt == 0 {
            '0'
        } else {
            '1'
        }
    };

    let oxygen = filter_by("Oxygen_1", &all_nums, &most_common_bit_closure);
    let co2 = filter_by("CO2", &all_nums, &least_common_bit_closure);

    println!("{}", oxygen * co2);
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
