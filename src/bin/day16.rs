#![feature(array_chunks)]

use nom::{bits::complete::take, IResult};

const INPUT: &str = include_str!("../../inputs/day16.txt");

fn main() {
    let packet: Packet = INPUT.parse().unwrap();
    println!("Answer 1: {}", packet.add_up_version_numbers());
    println!("Answer 2: {}", packet.value());
}

#[derive(Clone, Debug)]
struct Header {
    version: u8,
    packet_type: PacketType,
}

#[derive(Clone, Debug)]
enum PacketType {
    Sum,
    Product,
    Minimum,
    Maximum,
    Literal,
    GreatherThan,
    LessThan,
    Equal,
}

#[derive(Clone, Debug)]
struct Packet {
    header: Header,
    payload: Payload,
}

#[derive(Clone, Debug)]
enum Payload {
    Literal(usize),
    Operator(Vec<Packet>),
}

fn parse_literal_payload(mut input: (&[u8], usize)) -> IResult<(&[u8], usize), Payload> {
    let mut value = 0usize;
    loop {
        let (input2, digit): (_, u8) = take(5usize)(input)?;
        input = input2;
        value <<= 4;
        value |= (digit & 0b1111) as usize;
        if digit & 0b10000 != 0b10000 {
            break;
        }
    }
    Ok((input, Payload::Literal(value)))
}

fn parse_operator_payload0(input: (&[u8], usize)) -> IResult<(&[u8], usize), Payload> {
    let (mut input, total_length): (_, u16) = take(15usize)(input)?;
    let mut sub_packets = Vec::new();
    let mut length = 0;
    while let Ok((input2, sub_packet)) = parse_packet(input) {
        sub_packets.push(sub_packet);
        length += (input.0.len() * 8 - input.1 - (input2.0.len() * 8 - input2.1)) as u16;
        input = input2;
        if length >= total_length {
            break;
        }
    }

    Ok((input, Payload::Operator(sub_packets)))
}

fn parse_operator_payload1(input: (&[u8], usize)) -> IResult<(&[u8], usize), Payload> {
    let (mut input, num_sub_packets): (_, u16) = take(11usize)(input)?;
    let mut sub_packets = Vec::new();

    for _ in 0..num_sub_packets {
        let (input2, sub_packet) = parse_packet(input)?;
        sub_packets.push(sub_packet);
        input = input2;
    }

    Ok((input, Payload::Operator(sub_packets)))
}

fn parse_packet(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    let (input, version): (_, u8) = take(3usize)(input)?;
    let (mut input, type_id): (_, u8) = take(3usize)(input)?;

    let packet_type = match type_id {
        0 => PacketType::Sum,
        1 => PacketType::Product,
        2 => PacketType::Minimum,
        3 => PacketType::Maximum,
        4 => PacketType::Literal,
        5 => PacketType::GreatherThan,
        6 => PacketType::LessThan,
        7 => PacketType::Equal,
        _ => unreachable!(),
    };

    let payload = match packet_type {
        PacketType::Literal => {
            let (input2, payload) = parse_literal_payload(input)?;
            input = input2;
            payload
        }
        _ => {
            let (input2, length_type_id): (_, u8) = take(1usize)(input)?;
            input = input2;
            if length_type_id == 0 {
                let (input2, payload) = parse_operator_payload0(input)?;
                input = input2;
                payload
            } else {
                let (input2, payload) = parse_operator_payload1(input)?;
                input = input2;
                payload
            }
        }
    };

    Ok((
        input,
        Packet {
            header: Header {
                version,
                packet_type,
            },
            payload,
        },
    ))
}

impl std::str::FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s
            .trim()
            .chars()
            .collect::<Vec<_>>()
            .array_chunks::<2>()
            .map(|d| u8::from_str_radix(&d.iter().collect::<String>(), 16).unwrap())
            .collect::<Vec<_>>();

        let (_, packet) = parse_packet((&bytes, 0)).unwrap();
        Ok(packet)
    }
}

impl Packet {
    fn add_up_version_numbers(&self) -> usize {
        match &self.payload {
            Payload::Literal(_) => self.header.version as usize,
            Payload::Operator(sub_packets) => sub_packets
                .iter()
                .fold(self.header.version as usize, |acc, p| {
                    acc + p.add_up_version_numbers()
                }),
        }
    }

    fn value(&self) -> usize {
        match &self.payload {
            Payload::Literal(value) => *value,
            Payload::Operator(sub_packets) => {
                let mut sub_packet_values = sub_packets.iter().map(Self::value);
                match self.header.packet_type {
                    PacketType::Sum => sub_packet_values.sum(),
                    PacketType::Product => sub_packet_values.product(),
                    PacketType::Minimum => sub_packet_values.min().unwrap(),
                    PacketType::Maximum => sub_packet_values.max().unwrap(),
                    PacketType::Literal => unreachable!(),
                    PacketType::GreatherThan => {
                        assert_eq!(sub_packets.len(), 2);
                        if sub_packet_values.next().unwrap() > sub_packet_values.next().unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                    PacketType::LessThan => {
                        assert_eq!(sub_packets.len(), 2);
                        if sub_packet_values.next().unwrap() < sub_packet_values.next().unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                    PacketType::Equal => {
                        assert_eq!(sub_packets.len(), 2);
                        if sub_packet_values.next().unwrap() == sub_packet_values.next().unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_LITERAL: &str = "D2FE28";
    const EXAMPLE_OPERATOR0: &str = "38006F45291200";
    const EXAMPLE_OPERATOR1: &str = "EE00D40C823060";
    const EXAMPLE1: &str = "8A004A801A8002F478";
    const EXAMPLE2: &str = "620080001611562C8802118E34";
    const EXAMPLE3: &str = "C0015000016115A2E0802F182340";
    const EXAMPLE4: &str = "A0016C880162017C3686B18A3D4780";

    #[test]
    fn part1() {
        assert_eq!(
            EXAMPLE_LITERAL
                .parse::<Packet>()
                .unwrap()
                .add_up_version_numbers(),
            6
        );
        assert_eq!(
            EXAMPLE_OPERATOR0
                .parse::<Packet>()
                .unwrap()
                .add_up_version_numbers(),
            9
        );
        assert_eq!(
            EXAMPLE_OPERATOR1
                .parse::<Packet>()
                .unwrap()
                .add_up_version_numbers(),
            14
        );
        assert_eq!(
            EXAMPLE1.parse::<Packet>().unwrap().add_up_version_numbers(),
            16
        );
        assert_eq!(
            EXAMPLE2.parse::<Packet>().unwrap().add_up_version_numbers(),
            12
        );
        assert_eq!(
            EXAMPLE3.parse::<Packet>().unwrap().add_up_version_numbers(),
            23
        );
        assert_eq!(
            EXAMPLE4.parse::<Packet>().unwrap().add_up_version_numbers(),
            31
        );
    }

    #[test]
    fn part2() {
        assert_eq!("C200B40A82".parse::<Packet>().unwrap().value(), 3);
        assert_eq!("04005AC33890".parse::<Packet>().unwrap().value(), 54);
        assert_eq!("880086C3E88112".parse::<Packet>().unwrap().value(), 7);
        assert_eq!("CE00C43D881120".parse::<Packet>().unwrap().value(), 9);
        assert_eq!("D8005AC2A8F0".parse::<Packet>().unwrap().value(), 1);
        assert_eq!("F600BC2D8F".parse::<Packet>().unwrap().value(), 0);
        assert_eq!("9C005AC2F8F0".parse::<Packet>().unwrap().value(), 0);
        assert_eq!(
            "9C0141080250320F1802104A08"
                .parse::<Packet>()
                .unwrap()
                .value(),
            1
        );
    }
}
