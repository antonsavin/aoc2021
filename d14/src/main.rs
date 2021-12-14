use std::collections::HashMap;
use std::env;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn solve(part_1: bool) {
    let mut first_line = String::new();
    io::stdin().read_line(&mut first_line).unwrap();
    let formula = first_line.trim().chars().collect::<Vec<_>>();

    let reader = BufReader::new(io::stdin());
    let mut rules = HashMap::<(char, char), char>::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }

        let subst_fields = line.split(" -> ").collect::<Vec<_>>();
        let pair = subst_fields.get(0).unwrap();
        let pair_chars = pair.chars().collect::<Vec<_>>();
        let insert = subst_fields.get(1).unwrap();
        rules.insert(
            (*pair_chars.get(0).unwrap(), *pair_chars.get(1).unwrap()),
            insert.chars().next().unwrap(),
        );
    }

    // Init pair counts with 1 for each pair of adjacent characters.
    let mut pair_counts: HashMap<(char, char), usize> = formula
        .windows(2)
        .map(|pair| ((*pair.get(0).unwrap(), *pair.get(1).unwrap()), 1))
        .collect();

    let num_iterations = if part_1 { 10 } else { 40 };
    for _iter in 0..num_iterations {
        let mut new_pair_counts = HashMap::<(char, char), usize>::new();
        for (k, v) in pair_counts.iter() {
            let to_insert = rules.get(k);
            if let Some(c) = to_insert {
                *new_pair_counts.entry((k.0, *c)).or_insert(0) += v;
                *new_pair_counts.entry((*c, k.1)).or_insert(0) += v;
            } else {
                *new_pair_counts.entry(*k).or_insert(0) += v;
            }
        }
        pair_counts = new_pair_counts;
        // println!("After iter {} pair counts: {:?}", _iter + 1, pair_counts);
    }

    let mut char_counts = HashMap::<char, usize>::new();
    for ((c1, c2), cnt) in pair_counts.iter() {
        *char_counts.entry(*c1).or_insert(0) += cnt;
        *char_counts.entry(*c2).or_insert(0) += cnt;
    }

    // We counted all chars twice except for the first and the last.
    *char_counts.entry(*formula.get(0).unwrap()).or_insert(0) += 1;
    *char_counts
        .entry(*formula.get(formula.len() - 1).unwrap())
        .or_insert(0) += 1;

    let max_count = char_counts.values().max().unwrap();
    let min_count = char_counts.values().min().unwrap();
    println!("Max - min: {}", (max_count - min_count) / 2);
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
