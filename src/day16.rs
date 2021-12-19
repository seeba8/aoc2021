use std::str::FromStr;

pub fn solve() {
    let input = std::fs::read_to_string(format!(
        "resources/{}.txt",
        module_path!().split_once("::").unwrap().1
    ))
    .unwrap();
    let packet: Packet = input.parse().unwrap();
    println!("Day 16 part 1: {}", packet.sum_of_versions());
    println!("Day 16 part 2: {}", packet.get_value());
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Packet {
    Literal(Literal),
    Operator(Operator),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    version: u8,
    type_id: u8,
    payload: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Operator {
    version: u8,
    type_id: u8,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn parse(data: &[u8], used_bits: &mut usize) -> Packet {
        let version = Packet::use_bits(data, used_bits, 3);
        let type_id = Packet::use_bits(data, used_bits, 3);
        match type_id {
            4 => {
                let mut payload: usize = 0;
                loop {
                    let v = Packet::use_bits(data, used_bits, 5);
                    let first_bit_set = (v & 0b10000) > 0;
                    payload = (payload << 4) | (v & 0xf) as usize;
                    if !first_bit_set {
                        break;
                    }
                }
                Packet::Literal(Literal {
                    version,
                    type_id,
                    payload,
                })
            }
            _ => {
                let length_type_id = Packet::use_bits(data, used_bits, 1);
                if length_type_id == 0 {
                    let total_length_a = Packet::use_bits(data, used_bits, 7);
                    let total_length_b = Packet::use_bits(data, used_bits, 8);
                    let total_length: u16 =
                        ((total_length_a as u16) << 7) | (total_length_b as u16);
                    let mut sub_packets: Vec<Packet> = Vec::new();
                    let start_bit = *used_bits;
                    while *used_bits < start_bit + (total_length as usize) {
                        sub_packets.push(Packet::parse(data, used_bits));
                    }
                    Packet::Operator(Operator {
                        version,
                        type_id,
                        sub_packets,
                    })
                } else {
                    let number_subpackets_a = Packet::use_bits(data, used_bits, 3);
                    let number_subpackets_b = Packet::use_bits(data, used_bits, 8);
                    let number_subpackets: u16 =
                        ((number_subpackets_a as u16) << 7) | (number_subpackets_b as u16);
                    let mut sub_packets: Vec<Packet> =
                        Vec::with_capacity(number_subpackets as usize);
                    for _ in 0..number_subpackets {
                        sub_packets.push(Packet::parse(data, used_bits));
                    }
                    Packet::Operator(Operator {
                        version,
                        type_id,
                        sub_packets,
                    })
                }
            }
        }
    }

    fn use_bits(data: &[u8], used_bits: &mut usize, quantity: usize) -> u8 {
        if quantity > 8 {
            unimplemented!()
        }
        let v = if 8 - (*used_bits % 8) >= quantity {
            (data[*used_bits / 8] << (*used_bits % 8)) >> (8 - quantity)
        } else {
            // first get the remaining bits from the first byte and shift them so that they are in the correct position
            let buf = (data[*used_bits / 8] << (*used_bits % 8)) >> (8 - quantity);
            let used_bits = *used_bits + quantity;
            // now OR the remaining bits to it
            buf | (data[used_bits / 8] >> (8 - (used_bits % 8)))
        };
        *used_bits += quantity;
        v
    }

    pub fn sum_of_versions(&self) -> usize {
        match self {
            Packet::Literal(literal) => literal.version as usize,
            Packet::Operator(op) => {
                let s: usize = op.sub_packets.iter().map(|v| v.sum_of_versions()).sum();
                (op.version as usize) + s
            }
        }
    }

    pub fn get_value(&self) -> usize {
        match self {
            Packet::Literal(literal) => literal.payload,
            Packet::Operator(Operator {
                version: _,
                type_id,
                sub_packets,
            }) => match type_id {
                0 => sub_packets.iter().map(|sub| sub.get_value()).sum(),
                1 => sub_packets.iter().map(|sub| sub.get_value()).product(),
                2 => sub_packets.iter().map(|sub| sub.get_value()).min().unwrap(),
                3 => sub_packets.iter().map(|sub| sub.get_value()).max().unwrap(),
                5 => {
                    if sub_packets[0].get_value() > sub_packets[1].get_value() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if sub_packets[0].get_value() < sub_packets[1].get_value() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if sub_packets[0].get_value() == sub_packets[1].get_value() {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!(),
            },
        }
    }
}

impl FromStr for Packet {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = hex::decode(s.trim()).map_err(|_| "cannot parse input")?;
        Ok(Packet::parse(data.as_slice(), &mut 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_parses_literal() {
        let input = "D2FE28";
        let packet: Result<Packet, _> = input.parse();
        assert!(packet.is_ok());
        let packet = packet.unwrap();
        match packet {
            Packet::Literal(literal) => {
                assert_eq!(6, literal.version);
                assert_eq!(4, literal.type_id);
                assert_eq!(2021, literal.payload);
            }
            Packet::Operator(_) => assert!(false),
        };
    }

    #[test]
    fn it_parses_operator() {
        let input = "38006F45291200";
        let packet: Result<Packet, _> = input.parse();
        assert!(packet.is_ok());
        let packet = packet.unwrap();
        match packet {
            Packet::Literal(_) => assert!(false),
            Packet::Operator(operator) => {
                assert_eq!(1, operator.version);
                assert_eq!(6, operator.type_id);
                assert_eq!(2, operator.sub_packets.len());
            }
        };
    }

    #[test]
    fn it_parses_examples() {
        let input = "8A004A801A8002F478";
        let packet: Packet = input.parse().unwrap();
        assert_eq!(16, packet.sum_of_versions());
        let input = "620080001611562C8802118E34";
        let packet: Packet = input.parse().unwrap();
        assert_eq!(12, packet.sum_of_versions());
        let input = "C0015000016115A2E0802F182340";
        let packet: Packet = input.parse().unwrap();
        assert_eq!(23, packet.sum_of_versions());
        let input = "A0016C880162017C3686B18A3D4780";
        let packet: Packet = input.parse().unwrap();
        assert_eq!(31, packet.sum_of_versions());
    }

    #[test]
    fn it_calculates_packet_values() {
        assert_eq!(3, Packet::from_str("C200B40A82").unwrap().get_value());
        assert_eq!(54, Packet::from_str("04005AC33890").unwrap().get_value());
        assert_eq!(7, Packet::from_str("880086C3E88112").unwrap().get_value());
        assert_eq!(9, Packet::from_str("CE00C43D881120").unwrap().get_value());
        assert_eq!(1, Packet::from_str("D8005AC2A8F0").unwrap().get_value());
        assert_eq!(0, Packet::from_str("F600BC2D8F").unwrap().get_value());
        assert_eq!(0, Packet::from_str("9C005AC2F8F0").unwrap().get_value());
        assert_eq!(
            1,
            Packet::from_str("9C0141080250320F1802104A08")
                .unwrap()
                .get_value()
        );
    }
}
