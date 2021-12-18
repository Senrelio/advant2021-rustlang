use std::panic;

fn main() {
    let input = include_str!("../../inputs/day16_input");
    println!("day16 part1: {}", part1(input));
    println!("day16 part2: {}", part2(input));
}

fn pretreat(input: &str) -> Vec<char> {
    let mut seq = vec![];
    for c in input.chars() {
        let hex = u8::from_str_radix(&c.to_string(), 16).unwrap();
        let bin = format!("{:#06b}", hex);
        seq.extend(bin.chars().skip(2));
    }
    seq
}

fn part1(input: &str) -> u32 {
    let seq = pretreat(input);
    let packet = Packet::from_chars(&seq).0;
    version_sum(&packet)
}
fn part2(input: &str) -> u64 {
    let seq = pretreat(input);
    let packet = Packet::from_chars(&seq).0;
    packet.value()
}

fn version_sum(packet: &Packet) -> u32 {
    match packet {
        Packet::Literal(l) => l.version,
        Packet::Operator(o) => {
            o.version
                + o.sub_packets
                    .iter()
                    .map(|sub| version_sum(sub))
                    .sum::<u32>()
        }
    }
}

enum Packet {
    Literal(Literal),
    Operator(Operator),
}

struct Literal {
    version: u32,
    #[allow(dead_code)]
    type_id: u8,
    decimal: u64,
}

struct Operator {
    version: u32,
    type_id: u8,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn value(&self) -> u64 {
        match self {
            Packet::Literal(l) => l.decimal,
            Packet::Operator(p) => match p.type_id {
                0 => p.sub_packets.iter().map(|sub| sub.value()).sum(),
                1 => p
                    .sub_packets
                    .iter()
                    .map(|sub| sub.value())
                    .fold(1, |acc, v| acc * v),
                2 => p.sub_packets.iter().map(|sub| sub.value()).min().unwrap(),
                3 => p.sub_packets.iter().map(|sub| sub.value()).max().unwrap(),
                5 => {
                    if p.sub_packets[0].value() > p.sub_packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if p.sub_packets[0].value() < p.sub_packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if p.sub_packets[0].value() == p.sub_packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!(),
            },
        }
    }
    fn from_chars(chars: &[char]) -> (Packet, usize) {
        let version = u32::from_str_radix(&chars[..3].iter().collect::<String>(), 2).unwrap();
        let type_id = u8::from_str_radix(&chars[3..6].iter().collect::<String>(), 2).unwrap();
        if type_id == 4 {
            let mut bits = vec![];
            for (start, &bit) in chars.iter().enumerate().skip(6).step_by(5).into_iter() {
                bits.extend_from_slice(&chars[start + 1..start + 5]);
                if bit == '0' {
                    break;
                }
            }
            let decimal = u64::from_str_radix(String::from_iter(bits.iter()).as_str(), 2).unwrap();
            let packet = Packet::Literal(Literal {
                version,
                type_id,
                decimal,
            });
            let len = 6 + (bits.len() / 4) * 5;
            (packet, len)
        } else {
            let length_type = chars[6];
            match length_type {
                '0' => {
                    let bits_len =
                        usize::from_str_radix(String::from_iter(&chars[7..7 + 15]).as_str(), 2)
                            .unwrap();
                    let mut len_acc = 0;
                    let seq = &chars[7 + 15..];
                    let mut sub_packets = vec![];
                    'inner: loop {
                        if len_acc >= bits_len {
                            break 'inner;
                        }
                        let (packet, len) = Packet::from_chars(&seq[len_acc..]);
                        len_acc += len;
                        sub_packets.push(packet);
                    }
                    (
                        Packet::Operator(Operator {
                            version,
                            type_id,
                            sub_packets,
                        }),
                        7 + 15 + bits_len,
                    )
                }
                '1' => {
                    let subs_n =
                        usize::from_str_radix(String::from_iter(&chars[7..7 + 11]).as_str(), 2)
                            .unwrap();
                    let seq = &chars[7 + 11..];
                    let mut sub_packets = vec![];
                    let mut len_acc = 0;
                    for _ in 0..subs_n {
                        let (packet, len) = Packet::from_chars(&seq[len_acc..]);
                        sub_packets.push(packet);
                        len_acc += len;
                    }
                    (
                        Packet::Operator(Operator {
                            version,
                            type_id,
                            sub_packets,
                        }),
                        7 + 11 + len_acc,
                    )
                }
                _ => panic!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hex_test() {
        let c = 'A';
        let hex = u8::from_str_radix(&c.to_string(), 16).unwrap();
        assert_eq!(10, hex);
        let bin = format!("{:#06b}", hex);
        assert_eq!("0b1010", &bin);
    }
    #[test]
    fn part1_test() {
        let input = "D2FE28";
        assert_eq!(6, part1(input));
        let input = "8A004A801A8002F478";
        assert_eq!(16, part1(input));
        let input = "620080001611562C8802118E34";
        assert_eq!(12, part1(input));
        let input = "C0015000016115A2E0802F182340";
        assert_eq!(23, part1(input));
        let input = "A0016C880162017C3686B18A3D4780";
        assert_eq!(31, part1(input));
    }
    #[test]
    fn part2_test() {
        let set = [
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ];
        for (input, expect) in set {
            assert_eq!(expect, part2(input));
        }
    }
}
