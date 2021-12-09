use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn get_ij(hm: &Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
    if i < 0 || i >= hm.len() as i32 {
        return 100;
    }

    let row = hm.get(i as usize).unwrap();
    if j < 0 || j >= row.len() as i32 {
        return 100;
    }

    return *row.get(j as usize).unwrap();
}

fn read_height_map() -> Vec<Vec<i32>> {
    let reader = BufReader::new(io::stdin());
    let mut height_map: Vec<Vec<i32>> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();
        let mut row: Vec<i32> = vec![];
        for b in line.bytes() {
            row.push(b as i32 - '0' as i32);
        }
        height_map.push(row);
    }

    height_map
}

fn solve_part_1() {
    let height_map = read_height_map();

    let height: i32 = height_map.len() as i32;
    let width: i32 = height_map.get(0).unwrap().len() as i32;
    let mut total = 0;
    for i in 0..height {
        for j in 0..width {
            let h1 = get_ij(&height_map, i - 1, j);
            let h2 = get_ij(&height_map, i + 1, j);
            let h3 = get_ij(&height_map, i, j - 1);
            let h4 = get_ij(&height_map, i, j + 1);
            let cur = get_ij(&height_map, i, j);
            if cur < h1 && cur < h2 && cur < h3 && cur < h4 {
                total += cur + 1;
            }
        }
    }

    println!("Total: {}", total);
}

fn count_basin(hm: &Vec<Vec<i32>>, i_start: i32, j_start: i32) -> usize {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut queue = VecDeque::<(i32, i32)>::new();
    let height: i32 = hm.len() as i32;
    let width: i32 = hm.get(0).unwrap().len() as i32;
    queue.push_back((i_start, j_start));
    while !queue.is_empty() {
        let (i, j) = queue.pop_front().unwrap();
        if visited.contains(&(i, j)) {
            continue;
        }

        let this_height = get_ij(&hm, i, j);
        if this_height == 9 {
            continue;
        }

        visited.insert((i, j));

        if i > 0 && get_ij(&hm, i - 1, j) >= this_height {
            queue.push_back((i - 1, j));
        }
        if i + 1 < height && get_ij(&hm, i + 1, j) >= this_height {
            queue.push_back((i + 1, j));
        }
        if j > 0 && get_ij(&hm, i, j - 1) >= this_height {
            queue.push_back((i, j - 1));
        }
        if j + 1 < width && get_ij(&hm, i, j + 1) >= this_height {
            queue.push_back((i, j + 1));
        }
    }

    visited.len()
}

fn _flow_destinations(
    hm: &Vec<Vec<i32>>,
    i_start: i32,
    j_start: i32,
    final_destinations: &mut HashMap<(i32, i32), HashSet<(i32, i32)>>,
) {
    let mut this_visited = HashSet::<(i32, i32)>::new();
    let mut queue = VecDeque::<(i32, i32)>::new();
    let mut this_final_destinations = HashSet::<(i32, i32)>::new();
    let height: i32 = hm.len() as i32;
    let width: i32 = hm.get(0).unwrap().len() as i32;
    queue.push_back((i_start, j_start));

    while !queue.is_empty() {
        let (i, j) = queue.pop_front().unwrap();
        if this_visited.contains(&(i, j)) {
            continue;
        }

        this_visited.insert((i, j));
        let this_height = get_ij(&hm, i, j);
        if this_height == 9 {
            continue;
        }

        let mut is_final = true;
        if i > 0 && get_ij(&hm, i - 1, j) <= this_height {
            queue.push_back((i - 1, j));
            is_final = false;
        }
        if i + 1 < height && get_ij(&hm, i + 1, j) <= this_height {
            queue.push_back((i + 1, j));
            is_final = false;
        }
        if j > 0 && get_ij(&hm, i, j - 1) <= this_height {
            queue.push_back((i, j - 1));
            is_final = false;
        }
        if j + 1 < width && get_ij(&hm, i, j + 1) <= this_height {
            queue.push_back((i, j + 1));
            is_final = false;
        }

        if is_final {
            this_final_destinations.insert((i, j));
        }
    }

    final_destinations
        .entry((i_start, j_start))
        .or_insert(HashSet::<(i32, i32)>::new())
        .extend(this_final_destinations.clone());
}

fn _solve_part_2() {
    let height_map = read_height_map();

    let height: i32 = height_map.len() as i32;
    let width: i32 = height_map.get(0).unwrap().len() as i32;
    let mut final_destinations = HashMap::<(i32, i32), HashSet<(i32, i32)>>::new();

    for i in 0..height {
        for j in 0..width {
            _flow_destinations(&height_map, i, j, &mut final_destinations);
        }
    }

    println!("Final dest: {:?}", final_destinations);

    let dest_src_list: Vec<((i32, i32), (i32, i32))> = final_destinations
        .iter()
        .filter(|(_, dests)| dests.len() == 1)
        .map(|((i, j), dests)| (*dests.iter().next().unwrap(), (*i, *j)))
        .collect();

    let mut basin_sizes_map = HashMap::<(i32, i32), usize>::new();
    for (dest, _) in dest_src_list {
        *basin_sizes_map.entry(dest).or_insert(0) += 1;
    }

    let mut basin_sizes: Vec<_> = basin_sizes_map.values().collect();

    basin_sizes.sort();
    println!("Basins: {:?}", basin_sizes);

    let three_max = basin_sizes[basin_sizes.len() - 1]
        * basin_sizes[basin_sizes.len() - 2]
        * basin_sizes[basin_sizes.len() - 3];
    println!("Three max: {}", three_max);
}

fn solve_part_2_simpler() {
    let height_map = read_height_map();

    let height: i32 = height_map.len() as i32;
    let width: i32 = height_map.get(0).unwrap().len() as i32;
    let mut basin_sizes = vec![];

    for i in 0..height {
        for j in 0..width {
            let h1 = get_ij(&height_map, i - 1, j);
            let h2 = get_ij(&height_map, i + 1, j);
            let h3 = get_ij(&height_map, i, j - 1);
            let h4 = get_ij(&height_map, i, j + 1);
            let cur = get_ij(&height_map, i, j);
            if cur < h1 && cur < h2 && cur < h3 && cur < h4 {
                basin_sizes.push(count_basin(&height_map, i, j));
            }
        }
    }

    basin_sizes.sort();
    println!("Basins: {:?}", basin_sizes);

    let three_max = basin_sizes[basin_sizes.len() - 1]
        * basin_sizes[basin_sizes.len() - 2]
        * basin_sizes[basin_sizes.len() - 3];
    println!("Three max: {}", three_max);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_part_1 = args[1] == "1";
    if is_part_1 {
        solve_part_1();
    } else {
        solve_part_2_simpler();
    }
}
