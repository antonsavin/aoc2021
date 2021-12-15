use std::cmp::min;
use std::collections::HashMap;
use std::env;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn get_ij(hm: &Vec<Vec<i32>>, i: i32, j: i32, h: i32, w: i32, multiplier: i32) -> i32 {
    if i < 0 || i >= h * multiplier {
        panic!("Invalid i index");
    }

    let row = hm.get((i % h) as usize).unwrap();
    if j < 0 || j >= w * multiplier {
        panic!("Invalid j index");
    }

    let raw_value = *row.get((j % w) as usize).unwrap();
    return (raw_value + i / h + j / w - 1) % 9 + 1;
}

fn read_risk_map() -> Vec<Vec<i32>> {
    let reader = BufReader::new(io::stdin());
    let mut risk_map: Vec<Vec<i32>> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();
        let mut row: Vec<i32> = vec![];
        for b in line.bytes() {
            row.push(b as i32 - '0' as i32);
        }
        risk_map.push(row);
    }

    risk_map
}

fn solve(is_part_1: bool) {
    let risk_map = read_risk_map();
    let multiplier = if is_part_1 { 1 } else { 5 };
    let height: i32 = risk_map.len() as i32;
    let width: i32 = risk_map.get(0).unwrap().len() as i32;

    // println!("Risk map: {:?}", risk_map);
    let mut visited = HashMap::<(i32, i32), i32>::new();
    let mut cur_weights = HashMap::<(i32, i32), i32>::new();
    cur_weights.insert((0, 0), 0);
    while !visited.contains_key(&(height * multiplier - 1, width * multiplier - 1)) {
        let ((cur_i, cur_j), cur_weight) = cur_weights.iter().min_by_key(|(_, v)| *v).unwrap();
        let (cur_i, cur_j) = (*cur_i, *cur_j);
        // println!("Looking at {} {}", cur_i, cur_j);
        let cur_weight = *cur_weight;
        visited.insert((cur_i, cur_j), cur_weight);
        cur_weights.remove(&(cur_i, cur_j));

        for (di, dj) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let i1 = cur_i + di;
            let j1 = cur_j + dj;
            if i1 < 0 || i1 >= height * multiplier {
                continue;
            }

            if j1 < 0 || j1 >= width * multiplier {
                continue;
            }

            if visited.contains_key(&(i1, j1)) {
                continue;
            }

            let weight: &mut i32 = cur_weights.entry((i1, j1)).or_insert(i32::MAX);
            // println!(
            //     "   Coord {} {} prev weight {}, considering {}",
            //     i1,
            //     j1,
            //     weight,
            //     cur_weight + get_ij(&risk_map, i1, j1)
            // );
            *weight = min(
                *weight,
                cur_weight + get_ij(&risk_map, i1, j1, height, width, multiplier),
            );

            // println!(
            //     "   Coord {} {} now: {}",
            //     i1,
            //     j1,
            //     cur_weights.get(&(i1, j1)).unwrap()
            // );
        }
    }

    println!(
        "Lowest total risk: {}",
        visited
            .get(&(height * multiplier - 1, width * multiplier - 1))
            .unwrap()
    );
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
