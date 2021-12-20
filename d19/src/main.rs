use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use text_io::scan;

type Point = (i32, i32, i32);

#[derive(Debug, Eq, PartialEq)]
enum Axis {
    X,
    Y,
    Z,
}

// (sign, axis)
type AxisMap = (i32, Axis);

type Rotation = (AxisMap, AxisMap, AxisMap);

static ROTATIONS: [Rotation; 24] = [
    ((1, Axis::X), (1, Axis::Y), (1, Axis::Z)),
    ((1, Axis::X), (1, Axis::Z), (-1, Axis::Y)),
    ((1, Axis::X), (-1, Axis::Y), (-1, Axis::Z)),
    ((1, Axis::X), (-1, Axis::Z), (1, Axis::Y)),
    ((1, Axis::Y), (1, Axis::Z), (1, Axis::X)),
    ((1, Axis::Y), (1, Axis::X), (-1, Axis::Z)),
    ((1, Axis::Y), (-1, Axis::Z), (-1, Axis::X)),
    ((1, Axis::Y), (-1, Axis::X), (1, Axis::Z)),
    ((1, Axis::Z), (1, Axis::X), (1, Axis::Y)),
    ((1, Axis::Z), (1, Axis::Y), (-1, Axis::X)),
    ((1, Axis::Z), (-1, Axis::X), (-1, Axis::Y)),
    ((1, Axis::Z), (-1, Axis::Y), (1, Axis::X)),
    ((-1, Axis::X), (1, Axis::Z), (1, Axis::Y)),
    ((-1, Axis::X), (1, Axis::Y), (-1, Axis::Z)),
    ((-1, Axis::X), (-1, Axis::Z), (-1, Axis::Y)),
    ((-1, Axis::X), (-1, Axis::Y), (1, Axis::Z)),
    ((-1, Axis::Y), (1, Axis::X), (1, Axis::Z)),
    ((-1, Axis::Y), (1, Axis::Z), (-1, Axis::X)),
    ((-1, Axis::Y), (-1, Axis::X), (-1, Axis::Z)),
    ((-1, Axis::Y), (-1, Axis::Z), (1, Axis::X)),
    ((-1, Axis::Z), (1, Axis::Y), (1, Axis::X)),
    ((-1, Axis::Z), (1, Axis::X), (-1, Axis::Y)),
    ((-1, Axis::Z), (-1, Axis::Y), (-1, Axis::X)),
    ((-1, Axis::Z), (-1, Axis::X), (1, Axis::Y)),
];

#[derive(Debug, Clone)]
struct Scanner {
    id: i32,
    rot: &'static Rotation,
    beacons: Vec<Point>,
    dist: Point,
}

impl Scanner {
    fn new(id: i32, rot: &'static Rotation) -> Scanner {
        Scanner {
            id,
            rot,
            beacons: Vec::new(),
            dist: (0, 0, 0),
        }
    }
}

fn read_scanner(reader: &mut BufReader<io::Stdin>) -> Option<Scanner> {
    // println!("Reading scanner");
    let mut first_line = String::new();
    if let Ok(0) = reader.read_line(&mut first_line) {
        return None;
    }

    let scanner_id: i32;
    scan!(first_line.bytes() => "--- scanner {} ---", scanner_id);

    let mut scanner = Scanner::new(scanner_id, &ROTATIONS[0]);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            break;
        }

        let (x, y, z): (i32, i32, i32);
        scan!(line.bytes() => "{},{},{}", x, y, z);
        scanner.beacons.push((x, y, z));
    }

    // println!("Read scanner: {:?}", scanner);
    Some(scanner)
}

fn get_mapped_coord(coord: &Point, map: &AxisMap) -> i32 {
    let (x, y, z) = coord;

    match map {
        (sgn, Axis::X) => x * sgn,
        (sgn, Axis::Y) => y * sgn,
        (sgn, Axis::Z) => z * sgn,
    }
}

fn get_mapped_coords(coord: &Point, rot: &Rotation) -> Point {
    let (map_x, map_y, map_z) = rot;

    (
        get_mapped_coord(coord, map_x),
        get_mapped_coord(coord, map_y),
        get_mapped_coord(coord, map_z),
    )
}

fn convert_to_rot(s: &Scanner, rot: &'static Rotation) -> Scanner {
    let mut res = Scanner::new(s.id, rot);

    for b in &s.beacons {
        res.beacons.push(get_mapped_coords(&b, rot));
    }

    res
}

fn shift(beacons: &Vec<Point>, (x0, y0, z0): &Point) -> HashSet<Point> {
    let mut res = HashSet::new();
    for (x, y, z) in beacons {
        res.insert((x - x0, y - y0, z - z0));
    }

    res
}

// Returns (rot, dist)
fn match_scanners(s1: &Scanner, s2: &Scanner) -> Option<(&'static Rotation, Point)> {
    for b1 in &s1.beacons {
        let beacons1_shifted = shift(&s1.beacons, b1);

        for rot in &ROTATIONS {
            let s2_rotated = convert_to_rot(s2, rot);
            for b2 in &s2_rotated.beacons {
                let beacons2_shifted = shift(&s2_rotated.beacons, b2);

                let mut cnt_match = 0;
                for b2s in &beacons2_shifted {
                    if beacons1_shifted.contains(&b2s) {
                        cnt_match += 1;
                    }
                }

                if cnt_match >= 12 {
                    let (x1, y1, z1) = b1;
                    let (x2, y2, z2) = b2;
                    return Some((rot, (x1 - x2, y1 - y2, z1 - z2)));
                }
            }
        }
    }

    None
}

// Combine scanners with same rot and known distance
fn combine_scanners(s1: &Scanner, s2: &Scanner) -> Scanner {
    assert_eq!(s1.rot, s2.rot);

    let mut res = Scanner::new(-1, s1.rot);

    res.beacons = s1.beacons.clone();

    let s1_beacon_set = s1.beacons.iter().collect::<HashSet<_>>();
    let (dx, dy, dz) = s2.dist;
    let s2_beacons_shifted = shift(&s2.beacons, &(-dx, -dy, -dz));
    for b2 in s2_beacons_shifted {
        if !s1_beacon_set.contains(&b2) {
            res.beacons.push(b2);
        }
    }

    res
}

fn solve() {
    let mut reader = BufReader::new(io::stdin());

    let mut scanners = Vec::<Scanner>::new();

    loop {
        let maybe_scanner = read_scanner(&mut reader);
        if let Some(scanner) = maybe_scanner {
            scanners.push(scanner);
        } else {
            break;
        }
    }

    println!("Unmatched scanners: {}", scanners.len());

    let mut matched_scanners = Vec::<Scanner>::new();
    let first_scanner = scanners.get(0).unwrap();
    matched_scanners.push(first_scanner.clone());

    let mut remaining_scanners: Vec<&Scanner> = scanners.iter().skip(1).collect();

    while !remaining_scanners.is_empty() {
        let mut new_remaining_scanners: Vec<&Scanner> = Vec::new();

        for s in &remaining_scanners {
            let mut the_match = None;
            let mut dist_of_matched: Point = (0, 0, 0);

            for ms in &matched_scanners {
                let match_res = match_scanners(&ms, &s);
                if let Some(_) = match_res {
                    the_match = match_res;
                    dist_of_matched = ms.dist;
                    break;
                }
            }

            if let Some((rot, dist)) = the_match {
                let mut s = convert_to_rot(s, rot);
                s.rot = &ROTATIONS[0];
                let (mx, my, mz) = dist_of_matched;
                let (x, y, z) = dist;
                s.dist = (mx + x, my + y, mz + z);

                matched_scanners.push(s);
            } else {
                new_remaining_scanners.push(s);
            }
        }

        remaining_scanners = new_remaining_scanners;
    }

    println!("Matched scanners:");
    for s in &matched_scanners {
        println!("{:?}", s);
    }

    let mut the_combined_scanner = Scanner::new(-2, &ROTATIONS[0]);
    for s in &matched_scanners {
        the_combined_scanner = combine_scanners(&the_combined_scanner, &s);
    }

    println!("The combined scanner: {:?}", the_combined_scanner);
    println!("Total beacons: {}", the_combined_scanner.beacons.len());
    let mut max_dist = 0;
    for s1 in &matched_scanners {
        let (x1, y1, z1) = s1.dist;
        for s2 in &matched_scanners {
            let (x2, y2, z2) = s2.dist;
            let manh_dist = (x2 - x1).abs() + (y2 - y1).abs() + (z2 - z1).abs();
            if manh_dist > max_dist {
                max_dist = manh_dist;
            }
        }
    }
    println!("Max distance: {}", max_dist);
}

fn main() {
    solve();
}
