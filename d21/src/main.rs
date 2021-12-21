use std::collections::HashMap;
use std::env;
use std::io;
use text_io::scan;

fn read_players() -> (usize, usize) {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let player1_pos: usize;
    scan!(line.bytes() => "Player 1 starting position: {}", player1_pos);
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let player2_pos: usize;
    scan!(line.bytes() => "Player 2 starting position: {}", player2_pos);
    (player1_pos, player2_pos)
}

fn solve_part_1() {
    // Return (value, new dice val)
    fn roll(dice_val: usize) -> (usize, usize) {
        (dice_val * 3 + 6, dice_val + 3)
    }

    let (mut player1_pos, mut player2_pos) = read_players();

    let mut player1_score = 0;
    let mut player2_score = 0;

    let mut dice_val = 0;
    for i in 0..1000 {
        let (player1_pos_update, new_dice_val) = roll(dice_val);
        player1_pos = (player1_pos + player1_pos_update - 1) % 10 + 1;
        player1_score += player1_pos;
        dice_val = new_dice_val;
        if player1_score >= 1000 {
            let die_rolls = i * 6 + 3;
            println!(
                "Player 1 wins! Losing player score: {}, die rolls: {}, total: {}",
                player2_score,
                die_rolls,
                player2_score * die_rolls
            );
            break;
        }

        let (player2_pos_update, new_dice_val) = roll(dice_val);
        player2_pos = (player2_pos + player2_pos_update - 1) % 10 + 1;
        player2_score += player2_pos;
        dice_val = new_dice_val;
        if player2_score >= 1000 {
            let die_rolls = (i + 1) * 6;
            println!(
                "Player 2 wins! Losing player score: {}, die rolls: {}, total: {}",
                player1_score,
                die_rolls,
                player1_score * die_rolls
            );
            break;
        }
    }
}

// (score, pos) for player
type Score = (usize, usize);

// (player1, player2) -> count
type Counts = HashMap<(Score, Score), u128>;

fn update_scores_p1(counts: Counts) -> Counts {
    let mut new_counts: Counts = Counts::new();

    for (((s1, p1), (s2, p2)), cnt) in counts {
        for dice1 in 1..=3 {
            for dice2 in 1..=3 {
                for dice3 in 1..=3 {
                    let sum = dice1 + dice2 + dice3;
                    let new_point = (p1 + sum - 1) % 10 + 1;
                    *new_counts.entry(((s1 + new_point, new_point), (s2, p2))).or_insert(0) += cnt;
                }
            }
        }
    }

    new_counts
}

fn update_scores_p2(counts: Counts) -> Counts {
    let mut new_counts: Counts = Counts::new();

    for (((s1, p1), (s2, p2)), cnt) in counts {
        for dice1 in 1..=3 {
            for dice2 in 1..=3 {
                for dice3 in 1..=3 {
                    let sum = dice1 + dice2 + dice3;
                    let new_point = (p2 + sum - 1) % 10 + 1;
                    *new_counts.entry(((s1, p1), (s2 + new_point, new_point))).or_insert(0) += cnt;
                }
            }
        }
    }

    new_counts
}

fn solve_part_2() {
    let (player1_pos, player2_pos) = read_players();
    let mut counts = Counts::new();
    counts.insert(((0, player1_pos), (0, player2_pos)), 1);

    let mut p1_wins: u128 = 0;
    let mut p2_wins: u128 = 0;

    for turn in 1..20 {
        println!("==== Turn {}", turn);
        println!("Player 1");
        counts = update_scores_p1(counts);
        let total_counts: u128 = counts.values().sum();
        println!("Total counts: {}", total_counts);
        let p1_wins_this_turn: u128 = counts
            .iter()
            .filter_map(|(((s1, _), _), cnt)| if *s1 >= 21 { Some(cnt) } else { None })
            .sum();
        p1_wins += p1_wins_this_turn;
        println!("P1 wins this turn: {}, total: {}", p1_wins_this_turn, p1_wins);
        counts = counts.into_iter().filter(|(((s1, _), _), _)| *s1 < 21).collect();

        println!("Player 2");
        counts = update_scores_p2(counts);
        let total_counts: u128 = counts.values().sum();
        println!("Total counts: {}", total_counts);
        let p2_wins_this_turn: u128 = counts
            .iter()
            .filter_map(|((_, (s2, _)), cnt)| if *s2 >= 21 { Some(cnt) } else { None })
            .sum();
        p2_wins += p2_wins_this_turn;
        println!("P2 wins this turn: {}, total: {}", p2_wins_this_turn, p2_wins);
        counts = counts.into_iter().filter(|((_, (s2, _)), _)| *s2 < 21).collect();
        if total_counts == 0 {
            break;
        }
    }

    println!("Winner: {}", std::cmp::max(p1_wins, p2_wins));
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
