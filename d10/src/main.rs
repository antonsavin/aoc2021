use std::collections::HashMap;
use std::env;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn solve_part_1() {
    let char_scores: HashMap<char, u32> = [(')', 3), (']', 57), ('}', 1197), ('>', 25137)]
        .iter()
        .cloned()
        .collect();

    let matching_open: HashMap<char, char> = [(')', '('), (']', '['), ('}', '{'), ('>', '<')]
        .iter()
        .cloned()
        .collect();

    let reader = BufReader::new(io::stdin());
    let mut total_score = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();
        let mut stack = vec![];
        for c in line.chars() {
            if c == '(' || c == '[' || c == '{' || c == '<' {
                stack.push(c);
                continue;
            }

            if stack.is_empty() {
                total_score += char_scores.get(&c).unwrap();
                continue;
            }

            let top = *stack.last().unwrap();

            if top == *matching_open.get(&c).unwrap() {
                stack.pop();
            } else {
                total_score += char_scores.get(&c).unwrap();
                break;
            }
        }
    }

    println!("Total: {}", total_score);
}

fn solve_part_2() {
    let char_scores: HashMap<char, u64> = [('(', 1), ('[', 2), ('{', 3), ('<', 4)]
        .iter()
        .cloned()
        .collect();

    let matching_open: HashMap<char, char> = [(')', '('), (']', '['), ('}', '{'), ('>', '<')]
        .iter()
        .cloned()
        .collect();

    let reader = BufReader::new(io::stdin());

    let mut scores = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();
        let mut stack = vec![];
        let mut corrupt = false;
        for c in line.chars() {
            if c == '(' || c == '[' || c == '{' || c == '<' {
                stack.push(c);
                continue;
            }

            if stack.is_empty() {
                corrupt = true;
                break;
            }

            let top = *stack.last().unwrap();

            if top == *matching_open.get(&c).unwrap() {
                stack.pop();
            } else {
                corrupt = true;
                break;
            }
        }

        if corrupt {
            continue;
        }

        let score = stack
            .iter()
            .rfold(0, |x, cur| x * 5 + *char_scores.get(cur).unwrap());
        println!("Score: {}", score);
        scores.push(score);
    }

    scores.sort();
    println!("Median: {}", scores.get((scores.len() - 1) / 2).unwrap());
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
