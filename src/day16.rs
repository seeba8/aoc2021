use std::str::FromStr;

use hex;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Packet {
    Literal(Literal),
    Operator(Operator),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    version: u8,
    type_id: u8,
    payload: u128,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Operator {
    version: u8,
    type_id: u8,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn parse(data: &[u8], mut current_byte: usize, mut used_bits: usize) -> Packet {
        let version = if used_bits <= 5 {
            used_bits += 3;
            if used_bits >= 8 {
                current_byte += 1;
                used_bits = used_bits % 8;
            }
            data[current_byte] >> 5
        } else {
            // first get the remaining bits from the first byte and shift them so that they are in the correct position
            let buf = (data[current_byte] << used_bits) >> (3 + used_bits - 8);
            used_bits = (used_bits + 3) % 8;
            current_byte += 1;
            // now OR the remaining bits to it
            buf | (data[current_byte] >> (8 - used_bits))
        };
        let type_id = if used_bits <= 5 {
            let buf = (data[current_byte] << used_bits) >> (8 - 3);
            used_bits += 3;
            if used_bits >= 8 {
                current_byte += 1;
                used_bits = used_bits % 8;
            }
            buf
        } else {
            // first get the remaining bits from the first byte and shift them so that they are in the correct position
            let buf = (data[current_byte] << used_bits) >> (3 + used_bits - 8);
            used_bits = (used_bits + 3) % 8;
            current_byte += 1;
            // now OR the remaining bits to it
            buf | (data[current_byte] >> (8 - used_bits))
        };
        let mut payload: u128 = 0;
        while current_byte < data.len() {
            let v = if used_bits <= 3 {
                // entire next payload is in the same byte
                // we shift left to zero the part we have used. Then we shift right so that only 5 bits can ever be set
                let buf = (data[current_byte] << used_bits) >> (8 - 5);
                used_bits += 5;
                if used_bits >= 8 {
                    current_byte += 1;
                    used_bits = used_bits % 8;
                }
                buf
            } else {
                // first get the remaining bits from the first byte and shift them so that they are in the correct position
                let buf = (data[current_byte] << used_bits) >> (5 + used_bits - 8);
                used_bits = (used_bits + 5) % 8;
                current_byte += 1;
                // now OR the remaining bits to it
                buf | (data[current_byte] >> (8 - used_bits))
            };
            let first_bit_set = (v & 0b10000) > 0;
            payload = (payload << 4) | (v & 0xf) as u128;
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
}

impl FromStr for Packet {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = hex::decode(s.trim()).map_err(|op| "cannot parse input")?;
        Ok(Packet::parse(data.as_slice(), 0, 0))
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
        }
    }
}
