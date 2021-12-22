use std::cmp::{max, min};
use std::io::{BufRead, BufReader};
use text_io::scan;

#[derive(Clone, Debug)]
struct Cuboid {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
    z1: i64,
    z2: i64,
}

impl Cuboid {
    pub fn new(x1: i64, x2: i64, y1: i64, y2: i64, z1: i64, z2: i64) -> Cuboid {
        Cuboid { x1, x2, y1, y2, z1, z2 }
    }

    pub fn volume(&self) -> i64 {
        (self.x2 - self.x1) * (self.y2 - self.y1) * (self.z2 - self.z1)
    }

    fn diff(&self, c: &Cuboid) -> Vec<Cuboid> {
        let c1 = Cuboid::new(
            max(self.x1, c.x1),
            min(self.x2, c.x2),
            max(self.y1, c.y1),
            min(self.y2, c.y2),
            max(self.z1, c.z1),
            min(self.z2, c.z2),
        );

        if c1.x2 <= c1.x1 || c1.y2 <= c1.y1 || c1.z2 <= c1.z1 {
            return vec![self.clone()];
        }

        [
            Cuboid::new(self.x1, c1.x1, self.y1, self.y2, self.z1, self.z2),
            Cuboid::new(c1.x2, self.x2, self.y1, self.y2, self.z1, self.z2),
            Cuboid::new(c1.x1, c1.x2, self.y1, c1.y1, self.z1, self.z2),
            Cuboid::new(c1.x1, c1.x2, c1.y2, self.y2, self.z1, self.z2),
            Cuboid::new(c1.x1, c1.x2, c1.y1, c1.y2, self.z1, c1.z1),
            Cuboid::new(c1.x1, c1.x2, c1.y1, c1.y2, c1.z2, self.z2),
        ]
        .into_iter()
        .filter(|c| c.volume() > 0)
        .collect::<Vec<_>>()
    }
}

fn solve(part_1: bool) {
    let reader = BufReader::new(std::io::stdin());

    let mut cuboids = Vec::<Cuboid>::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let on_off: String;
        let (mut x1, mut x2, mut y1, mut y2, mut z1, mut z2): (i64, i64, i64, i64, i64, i64);
        scan!(line.bytes() => "{} x={}..{},y={}..{},z={}..{}", on_off, x1, x2, y1, y2, z1, z2);
        if part_1 {
            x1 = max(x1, -50);
            x2 = min(x2, 50);
            y1 = max(y1, -50);
            y2 = min(y2, 50);
            z1 = max(z1, -50);
            z2 = min(z2, 50);
        }

        if x1 > x2 || y1 > y2 || z1 > z2 {
            continue;
        }

        let c = Cuboid::new(x1, x2 + 1, y1, y2 + 1, z1, z2 + 1);
        let turn_on = on_off == "on";

        let mut new_cuboids = Vec::<Cuboid>::new();
        for old_cube in cuboids.into_iter() {
            let new_cubes: Vec<Cuboid>;
            new_cubes = old_cube.diff(&c);
            new_cuboids.extend(new_cubes);
        }

        if turn_on {
            new_cuboids.push(c);
        }

        cuboids = new_cuboids;
    }

    let res = cuboids.iter().map(|c| c.volume()).sum::<i64>();
    println!("Total cubes: {}", res);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let is_part_1 = args[1] == "1";
    solve(is_part_1);
}
