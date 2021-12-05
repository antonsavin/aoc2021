use std::collections::HashSet;
use std::env;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

type Table = [[u32; 5]; 5];

struct BingoCard {
    nums: Table,
    checks: [[bool; 5]; 5],
}

impl BingoCard {
    pub fn new() -> BingoCard {
        BingoCard {
            nums: [[0; 5]; 5],
            checks: [[false; 5]; 5],
        }
    }

    pub fn check_num(&mut self, num: u32) -> bool {
        for i in 0..5 {
            for j in 0..5 {
                if self.nums[i][j] != num {
                    continue;
                }

                self.checks[i][j] = true;
                let mut full_row = true;
                for j1 in 0..5 {
                    if !self.checks[i][j1] {
                        full_row = false;
                        break;
                    }
                }

                let mut full_col = true;
                for i1 in 0..5 {
                    if !self.checks[i1][j] {
                        full_col = false;
                        break;
                    }
                }

                return full_row || full_col;
            }
        }

        false
    }

    pub fn sum_unmarked(&self) -> u32 {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.checks[i][j] {
                    sum += self.nums[i][j];
                }
            }
        }

        sum
    }
}

fn read_nums() -> Vec<u32> {
    let mut first_line = String::new();
    io::stdin().read_line(&mut first_line).unwrap();

    let nums: Vec<_> = first_line
        .trim()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    nums
}

fn read_tables() -> Vec<BingoCard> {
    let mut tables: Vec<BingoCard> = vec![];

    let reader = BufReader::new(io::stdin());
    let mut idx = 0;

    for line in reader.lines() {
        idx += 1;
        if idx == 1 {
            continue;
        }

        if idx == 2 {
            let cur_table = BingoCard::new();
            tables.push(cur_table);
        }

        let line = line.unwrap();
        let line_nums: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let last_table_idx = tables.len() - 1;
        let cur_table = tables.get_mut(last_table_idx).unwrap();
        for i in 0..5 {
            cur_table.nums[idx - 2][i] = line_nums[i];
        }

        if idx == 6 {
            idx = 0;
        }
    }

    tables
}

fn solve_part_1() {
    let nums = read_nums();
    let mut tables = read_tables();
    println!("Read {} tables", tables.len());

    for num in nums {
        for table in &mut tables {
            if table.check_num(num) {
                println!("Score: {}", table.sum_unmarked() * num);
                return;
            }
        }
    }
}

fn solve_part_2() {
    let nums = read_nums();
    let mut tables = read_tables();
    println!("Read {} tables", tables.len());
    let mut not_won_yet = HashSet::<usize>::new();
    not_won_yet.extend(0..tables.len());

    for num in nums {
        for (idx, table) in tables.iter_mut().enumerate() {
            if !not_won_yet.contains(&idx) {
                continue;
            }

            if table.check_num(num) {
                not_won_yet.remove(&idx);
                if not_won_yet.is_empty() {
                    println!("Last winning score: {}", table.sum_unmarked() * num);
                    return;
                }
            }
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
