use std::collections::HashSet;
use std::env;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn solve(part_1: bool) {
    let reader = BufReader::new(io::stdin());

    let mut reading_folds = false;
    let mut dots = HashSet::<(i32, i32)>::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if reading_folds {
            // println!("Dots: {} {:?}", dots.len(), dots);
            let fields: Vec<_> = line.trim().split(' ').collect();
            let spec = fields.get(2).unwrap();
            let spec_fields: Vec<_> = spec.split('=').collect();
            let coord = spec_fields.get(0).unwrap();
            let value = spec_fields.get(1).unwrap().parse::<i32>().unwrap();
            println!("Splitting along {} value {}", coord, value);

            let mut new_dots = HashSet::<(i32, i32)>::new();
            for (x, y) in dots.iter() {
                if coord == &"x" {
                    if *x < value {
                        new_dots.insert((*x, *y));
                    } else {
                        new_dots.insert((value - (*x - value), *y));
                    }
                } else if coord == &"y" {
                    if *y < value {
                        new_dots.insert((*x, *y));
                    } else {
                        new_dots.insert((*x, value - (*y - value)));
                    }
                }
            }

            if part_1 {
                println!("After first fold: {}", new_dots.len());
                break;
            }

            dots = new_dots;
        } else {
            if line.len() == 0 {
                reading_folds = true;
                continue;
            }

            let fields: Vec<_> = line.trim().split(',').collect();
            let x = fields.get(0).unwrap().parse::<i32>().unwrap();
            let y = fields.get(1).unwrap().parse::<i32>().unwrap();
            dots.insert((x, y));
        }
    }

    if part_1 {
        return;
    }

    // println!("Dots: {} {:?}", dots.len(), dots);
    let max_x = dots.iter().map(|(x, _)| *x).max().unwrap();
    let max_y = dots.iter().map(|(_, y)| *y).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
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
