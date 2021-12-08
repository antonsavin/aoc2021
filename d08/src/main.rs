use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn solve_part_1() {
    let reader = BufReader::new(io::stdin());

    let mut cnt = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let input_output: Vec<&str> = line.split('|').collect();
        let output = input_output.get(1).unwrap().trim().split_whitespace();
        for s in output {
            if s.len() == 2 || s.len() == 4 || s.len() == 3 || s.len() == 7 {
                cnt += 1;
            }
        }
    }

    println!("Total [1478]: {}", cnt);
}

fn deduce_char_mapping(input: Vec<&str>) -> HashMap<char, char> {
    let mut str_1 = HashSet::new();
    let mut str_4 = HashSet::new();
    let mut char_counts = HashMap::<char, u32>::new();
    for s in input {
        for c in s.chars() {
            *char_counts.entry(c).or_insert(0) += 1;
        }

        match s.len() {
            2 => {
                str_1 = s.chars().collect();
            }
            4 => {
                str_4 = s.chars().collect();
            }
            _ => (),
        }
    }

    assert_eq!(char_counts.len(), 7);
    let mut char_mapping = HashMap::<char, char>::new();

    for (k, v) in char_counts.iter() {
        match *v {
            8 => {
                if str_1.contains(k) {
                    char_mapping.insert(*k, 'c');
                } else {
                    char_mapping.insert(*k, 'a');
                }
            }
            6 => {
                char_mapping.insert(*k, 'b');
            }
            7 => {
                if str_4.contains(k) {
                    char_mapping.insert(*k, 'd');
                } else {
                    char_mapping.insert(*k, 'g');
                }
            }
            4 => {
                char_mapping.insert(*k, 'e');
            }
            9 => {
                char_mapping.insert(*k, 'f');
            }
            _ => {
                panic!("Unexpected char count: {}", *v);
            }
        }
    }

    assert_eq!(
        char_mapping.len(),
        7,
        "Invalid char mapping len: {}",
        char_mapping.len()
    );

    // println!("Mapping: {:?}", char_mapping);
    char_mapping
}

fn convert_to_num(
    s: &str,
    char_mapping: &HashMap<char, char>,
    canon_map: &HashMap<&str, u32>,
) -> u32 {
    let mut chars: Vec<char> = s.chars().map(|c| *char_mapping.get(&c).unwrap()).collect();
    chars.sort();
    let canon_s: String = chars.into_iter().collect();
    return *canon_map.get(&canon_s as &str).unwrap();
}

fn solve_part_2() {
    let canon_map: HashMap<&str, u32> = [
        ("abcefg", 0),
        ("cf", 1),
        ("acdeg", 2),
        ("acdfg", 3),
        ("bcdf", 4),
        ("abdfg", 5),
        ("abdefg", 6),
        ("acf", 7),
        ("abcdefg", 8),
        ("abcdfg", 9),
    ]
    .iter()
    .cloned()
    .collect();

    let reader = BufReader::new(io::stdin());
    let mut total = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let input_output: Vec<&str> = line.split('|').collect();
        let input = input_output
            .get(0)
            .unwrap()
            .trim()
            .split_whitespace()
            .collect();
        let char_mapping = deduce_char_mapping(input);

        let output: Vec<&str> = input_output
            .get(1)
            .unwrap()
            .trim()
            .split_whitespace()
            .collect();
        let mut num = 0;
        for s in output {
            let digit = convert_to_num(s, &char_mapping, &canon_map);
            num = num * 10 + digit;
        }
        // println!("Num: {}", num);
        total += num;
    }

    println!("Total: {}", total);
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
