use std::env;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

// #[derive(Debug)]
#[derive(Clone)]
struct TreeNode {
    value: Option<i32>,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

struct CheckExplodeResult {
    exploded: bool,
    just_exploded: bool,
    to_add_left: Option<i32>,
    to_add_right: Option<i32>,
}

impl TreeNode {
    pub fn new(s: &str) -> Box<TreeNode> {
        let s = s.trim();
        let mut chars = s.chars();
        TreeNode::read_tree(&mut chars)
    }

    fn from_number(x: i32) -> Box<TreeNode> {
        Box::new(TreeNode {
            value: Some(x),
            left: None,
            right: None,
        })
    }

    fn from_pair(left: Box<TreeNode>, right: Box<TreeNode>) -> Box<TreeNode> {
        Box::new(TreeNode {
            value: None,
            left: Some(left),
            right: Some(right),
        })
    }

    fn read_tree(chars: &mut std::str::Chars) -> Box<TreeNode> {
        let c = chars.next().unwrap();
        match c {
            '0'..='9' => TreeNode::from_number(c as i32 - '0' as i32),
            '[' => {
                let left = TreeNode::read_tree(chars);
                assert_eq!(chars.next().unwrap(), ',');
                let right = TreeNode::read_tree(chars);
                assert_eq!(chars.next().unwrap(), ']');
                TreeNode::from_pair(left, right)
            }
            _ => {
                panic!("Invalid character '{}'", c);
            }
        }
    }

    fn reduce(&mut self) {
        // println!("    Before reduce: {}", self);
        let mut changed = true;

        while changed {
            changed = false;

            loop {
                let check_explode_res = self.check_explode(0);
                if !check_explode_res.exploded {
                    break;
                }
                changed = true;
                // println!("    After explode: {}", self);
            }

            if self.check_split() {
                changed = true;
                // println!("      After split: {}", self);
            }
        }

        // println!("     After reduce: {}", self);
    }

    fn is_number(&self) -> bool {
        if let Some(_) = self.value {
            true
        } else {
            false
        }
    }

    fn get_number(&self) -> i32 {
        if let Some(x) = self.value {
            x
        } else {
            panic!("Can't get number from pair")
        }
    }

    fn check_explode(&mut self, level: i32) -> CheckExplodeResult {
        if let (Some(left), Some(right)) = (&mut self.left, &mut self.right) {
            if level >= 4 {
                if !left.is_number() || !right.is_number() {
                    panic!("Non-number pair on level 5: {}", self);
                }

                if left.is_number() && right.is_number() {
                    return CheckExplodeResult {
                        exploded: true,
                        just_exploded: true,
                        to_add_left: Some(left.get_number()),
                        to_add_right: Some(right.get_number()),
                    };
                }
            }

            let left_result = left.check_explode(level + 1);
            if left_result.exploded {
                if left_result.just_exploded {
                    self.left = Some(TreeNode::from_number(0));
                }

                right.add_leftmost(left_result.to_add_right);
                return CheckExplodeResult {
                    exploded: true,
                    just_exploded: false,
                    to_add_left: left_result.to_add_left,
                    to_add_right: None,
                };
            }

            let right_result = right.check_explode(level + 1);
            if right_result.exploded {
                if right_result.just_exploded {
                    self.right = Some(TreeNode::from_number(0));
                }

                // if let Some(toadd) = right_result.to_add_left {
                //     println!("Add rightmost {} to {}", toadd, left);
                // }
                left.add_rightmost(right_result.to_add_left);
                return CheckExplodeResult {
                    exploded: true,
                    just_exploded: false,
                    to_add_left: None,
                    to_add_right: right_result.to_add_right,
                };
            }

            return CheckExplodeResult {
                exploded: false,
                just_exploded: false,
                to_add_left: None,
                to_add_right: None,
            };
        } else {
            return CheckExplodeResult {
                exploded: false,
                just_exploded: false,
                to_add_left: None,
                to_add_right: None,
            };
        };
    }

    fn check_split(&mut self) -> bool {
        if let (Some(left), Some(right)) = (&mut self.left, &mut self.right) {
            if left.is_number() {
                let left_num = left.get_number();
                if left_num >= 10 {
                    self.left = Some(TreeNode::from_pair(
                        TreeNode::from_number(left_num / 2),
                        TreeNode::from_number(left_num - left_num / 2),
                    ));
                    return true;
                }
            } else if left.check_split() {
                return true;
            }

            if right.is_number() {
                let right_num = right.get_number();
                if right_num >= 10 {
                    self.right = Some(TreeNode::from_pair(
                        TreeNode::from_number(right_num / 2),
                        TreeNode::from_number(right_num - right_num / 2),
                    ));
                    return true;
                }
            } else {
                return right.check_split();
            }
        }

        false
    }

    fn add_leftmost(&mut self, maybe_value: Option<i32>) {
        if let Some(value) = maybe_value {
            if let Some(x) = self.value {
                self.value = Some(x + value);
            } else if let Some(left) = &mut self.left {
                if left.is_number() {
                    let left_value = left.get_number();
                    self.left = Some(TreeNode::from_number(left_value + value));
                } else {
                    left.add_leftmost(maybe_value);
                }
            } else {
                panic!("Can't be here")
            }
        }
    }

    fn add_rightmost(&mut self, maybe_value: Option<i32>) {
        if let Some(value) = maybe_value {
            if let Some(x) = self.value {
                self.value = Some(x + value);
            } else if let Some(right) = &mut self.right {
                if right.is_number() {
                    let right_value = right.get_number();
                    self.right = Some(TreeNode::from_number(right_value + value));
                } else {
                    right.add_rightmost(maybe_value);
                }
            } else {
                panic!("Can't be here")
            }
        }
    }

    fn magnitude(&self) -> i32 {
        if let Some(x) = self.value {
            x
        } else if let (Some(left), Some(right)) = (&self.left, &self.right) {
            3 * left.magnitude() + 2 * right.magnitude()
        } else {
            panic!("Can't be here")
        }
    }
}

impl std::fmt::Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let (Some(left), Some(right)) = (&self.left, &self.right) {
            write!(f, "[{},{}]", left, right)
        } else if let Some(x) = self.value {
            write!(f, "{}", x)
        } else {
            panic!("Invalid object")
        }
    }
}

fn add(left: Box<TreeNode>, right: Box<TreeNode>) -> Box<TreeNode> {
    let mut new_root = TreeNode::from_pair(left, right);
    new_root.reduce();
    new_root
}

fn solve_part_1() {
    let reader = BufReader::new(io::stdin());
    let mut maybe_prev_root: Option<Box<TreeNode>> = None;

    for line in reader.lines() {
        let line = line.unwrap();
        let root = TreeNode::new(&line);

        println!("Tree: {}", root);
        if let Some(prev_root) = maybe_prev_root {
            maybe_prev_root = Some(add(prev_root, root));
            if let Some(rr) = &maybe_prev_root {
                println!("  Current sum: {}", rr)
            };
        } else {
            maybe_prev_root = Some(root);
        }
    }

    if let Some(rr) = maybe_prev_root {
        println!("Total magnitude: {}", rr.magnitude());
    }
}

fn solve_part_2() {
    let reader = BufReader::new(io::stdin());

    let trees = reader
        .lines()
        .map(|line| TreeNode::new(&line.unwrap()))
        .collect::<Vec<_>>();

    let mut max_magnitude = 0;
    for t1 in trees.iter() {
        for t2 in trees.iter() {
            let mag = add(t1.clone(), t2.clone()).magnitude();
            if mag > max_magnitude {
                max_magnitude = mag;
            }
        }
    }

    println!("Max pairwise magnitude: {}", max_magnitude);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let sum = add(
            TreeNode::new("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"),
            TreeNode::new("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"),
        );
        let sum_repr = format!("{}", sum);
        assert_eq!(sum_repr, "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");

        let sum = add(sum, TreeNode::new("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]"));
        let sum_repr = format!("{}", sum);
        assert_eq!(
            sum_repr,
            "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]"
        );
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(TreeNode::new("[[1,2],[[3,4],5]]").magnitude(), 143);
        assert_eq!(TreeNode::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude(), 1384);
        assert_eq!(
            TreeNode::new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(),
            3488
        );
    }
}
