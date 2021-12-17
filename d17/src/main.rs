use std::env;
use std::io;
use text_io::scan;

fn simulate(vx: i32, vy: i32, x1: i32, x2: i32, y1: i32, y2: i32) -> bool {
    let mut x = 0;
    let mut y = 0;
    let mut cur_vx = vx;
    let mut cur_vy = vy;

    loop {
        if x >= x1 && x <= x2 && y >= y1 && y <= y2 {
            return true;
        }

        if x > x2 || y < y1 {
            return false;
        }

        x += cur_vx;
        y += cur_vy;

        if cur_vx > 0 {
            cur_vx -= 1;
        }

        cur_vy -= 1;
    }
}

fn solve(part_1: bool) {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let (x1, y1, x2, y2): (i32, i32, i32, i32);
    scan!(line.bytes() => "target area: x={}..{}, y={}..{}", x1, x2, y1, y2);
    //println!("{} {} {} {}", x1, x2, y1, y2);

    // This only works if the target area is below our level
    assert!(y1 < 0);

    // The last vertical step is for (-y1) down, therefore we launch by (-y1 - 1) up.
    let max_vertical_speed = -y1 - 1;

    // We also need that the area is sufficently close horizontally, so that the last few steps are strictly vertical down.
    let num_vertical_steps = max_vertical_speed * 2 + 1;
    assert!(x2 <= num_vertical_steps * (num_vertical_steps + 1) / 2);

    if part_1 {
        let max_height = max_vertical_speed * (max_vertical_speed + 1) / 2;
        println!("Max height: {}", max_height);
    } else {
        let mut valid_cnt = 0;
        for vx in 1..=x2 {
            for vy in y1..=max_vertical_speed {
                if simulate(vx, vy, x1, x2, y1, y2) {
                    valid_cnt += 1;
                }
            }
        }

        println!("Velocity count: {}", valid_cnt);
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
