// use std::collections::HashMap;
use std::env;
use std::io;

type Bits = Vec<u8>;

pub fn get_bits(line: &str) -> Bits {
    let line = line.trim();
    line.chars()
        .flat_map(|c| {
            let x = u8::from_str_radix(&String::from(c), 16).unwrap();
            [(x >> 3) & 1, (x >> 2) & 1, (x >> 1) & 1, x & 1]
        })
        .collect::<Bits>()
}

#[derive(Debug)]
struct Packet {
    version: i64,
    packet_type: i64,
    value: Option<i64>,
    size: i64,
    length_type: Option<i64>,
    children_count_or_size: i64,
    children: Vec<Packet>,
}

impl Packet {
    fn new(bits_iter: &mut std::slice::Iter<u8>, _indent: usize) -> Packet {
        let _prefix = format!("{: <1$}", "", _indent * 2);
        // println!("{}Reading packet", _prefix);

        let mut packet = Packet {
            version: 0,
            packet_type: 0,
            value: None,
            size: 0,
            length_type: None,
            children_count_or_size: 0,
            children: Vec::new(),
        };

        packet.version = packet.read_raw(bits_iter, 3);
        packet.packet_type = packet.read_raw(bits_iter, 3);

        if packet.packet_type == 4 {
            packet.value = Some(packet.read_number(bits_iter));
        } else {
            packet.length_type = Some(packet.read_raw(bits_iter, 1));

            if packet.length_type == Some(0) {
                // println!("{}Reading children bit count", _prefix);
                let mut children_bit_count = packet.read_raw(bits_iter, 15);
                packet.children_count_or_size = children_bit_count;
                // println!("{}Packet so far: {:?}, reading children by bytes", _prefix, packet);

                while children_bit_count > 0 {
                    let child = Packet::new(bits_iter, _indent + 1);
                    packet.size += child.size;
                    children_bit_count -= child.size;
                    packet.children.push(child);
                }
            } else {
                let child_count = packet.read_raw(bits_iter, 11);
                packet.children_count_or_size = child_count;
                // println!("{}Packet so far: {:?}, reading children by count", _prefix, packet);
                for _ in 0..child_count {
                    let child = Packet::new(bits_iter, _indent + 1);
                    packet.size += child.size;
                    packet.children.push(child);
                }
            }
        }

        packet
    }

    fn calc_value(&self) -> i64 {
        match self.packet_type {
            0 => self.children.iter().map(|c| c.calc_value()).sum(),
            1 => self.children.iter().map(|c| c.calc_value()).product(),
            2 => self.children.iter().map(|c| c.calc_value()).min().unwrap(),
            3 => self.children.iter().map(|c| c.calc_value()).max().unwrap(),
            4 => self.value.unwrap(),
            5 => {
                if self.children.get(0).unwrap().calc_value() > self.children.get(1).unwrap().calc_value() {
                    1
                } else {
                    0
                }
            }
            6 => {
                if self.children.get(0).unwrap().calc_value() < self.children.get(1).unwrap().calc_value() {
                    1
                } else {
                    0
                }
            }
            7 => {
                if self.children.get(0).unwrap().calc_value() == self.children.get(1).unwrap().calc_value() {
                    1
                } else {
                    0
                }
            }

            _ => panic!("Invalid packet type"),
        }
    }

    fn read_raw(&mut self, bits_iter: &mut std::slice::Iter<u8>, bit_cnt: i64) -> i64 {
        let mut res: i64 = 0;
        for _ in 0..bit_cnt {
            let bit = *bits_iter.next().unwrap() as i64;
            res = (res << 1) | bit;
            // print!("bit {}, cur: {}; ", bit, res);
        }
        // println!();
        self.size += bit_cnt;
        res
    }

    fn read_number(&mut self, bits_iter: &mut std::slice::Iter<u8>) -> i64 {
        let mut res: i64 = 0;

        loop {
            let next_num = self.read_raw(bits_iter, 5);
            res = (res << 4) | (next_num & 0xF);
            if next_num & 0x10 == 0 {
                break;
            }
        }

        res
    }
}

fn version_sum(packet: &Packet) -> i64 {
    return packet.version + packet.children.iter().map(|c| version_sum(c)).sum::<i64>();
}

fn solve(part_1: bool) {
    let mut first_line = String::new();
    io::stdin().read_line(&mut first_line).unwrap();
    let bits = get_bits(&first_line);
    // println!("Bits : {:?}", bits);

    let mut bits_iter = bits.iter();
    let packet = Packet::new(&mut bits_iter, 0);
    // println!("Packet: {:?}", packet);
    if part_1 {
        println!("Version sum: {}", version_sum(&packet));
    } else {
        println!("Calculated value: {}", packet.calc_value());
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

#[cfg(test)]
mod tests {
    use super::*;

    fn version_sum_from_str(input: &str) -> i64 {
        let bits = get_bits(input);
        let mut bits_iter = bits.iter();
        let packet = Packet::new(&mut bits_iter, 0);
        version_sum(&packet)
    }

    fn calc_value_from_str(input: &str) -> i64 {
        let bits = get_bits(input);
        let mut bits_iter = bits.iter();
        let packet = Packet::new(&mut bits_iter, 0);
        packet.calc_value()
    }

    #[test]
    fn test_version_sum() {
        assert_eq!(version_sum_from_str("8A004A801A8002F478"), 16);
        assert_eq!(version_sum_from_str("620080001611562C8802118E34"), 12);
        assert_eq!(version_sum_from_str("C0015000016115A2E0802F182340"), 23);
        assert_eq!(version_sum_from_str("A0016C880162017C3686B18A3D4780"), 31);
    }

    #[test]
    fn test_calc_value() {
        assert_eq!(calc_value_from_str("C200B40A82"), 3);
        assert_eq!(calc_value_from_str("04005AC33890"), 54);
        assert_eq!(calc_value_from_str("880086C3E88112"), 7);
        assert_eq!(calc_value_from_str("CE00C43D881120"), 9);
        assert_eq!(calc_value_from_str("D8005AC2A8F0"), 1);
        assert_eq!(calc_value_from_str("F600BC2D8F"), 0);
        assert_eq!(calc_value_from_str("9C005AC2F8F0"), 0);
        assert_eq!(calc_value_from_str("9C0141080250320F1802104A08"), 1);
    }
}
