use std::{iter, str::from_utf8, vec::Vec};

use crate::utils::*;
use bitvec::field::BitField;
use bitvec::prelude::*;

const LITERAL: u8 = 4;
const SUM: u8 = 0;
const PROD: u8 = 1;
const MIN: u8 = 2;
const MAX: u8 = 3;
const GREATER: u8 = 5;
const LESSER: u8 = 6;
const EQUALTO: u8 = 7;

const LEN_15_BITS: bool = false;
const LEN_11_BITS: bool = true;

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketData {
    Literal(usize),
    Operator(Vec<Packet>),
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Packet {
    version: u8,
    type_id: u8,
    size: usize,
    data: PacketData,
}

impl Packet {
    fn new(version: u8, type_id: u8, size: usize, data: PacketData) -> Self {
        Self {
            version,
            type_id,
            size,
            data,
        }
    }
    fn literal(version: u8, size: usize, data: usize) -> Self {
        Self {
            version,
            type_id: LITERAL,
            size,
            data: PacketData::Literal(data),
        }
    }
    fn operator(version: u8, type_id: u8, size: usize, data: Vec<Packet>) -> Self {
        Self {
            version,
            type_id,
            size,
            data: PacketData::Operator(data),
        }
    }

    fn fold<T, F>(&self, state: T, f: F) -> T
    where
        F: Fn(T, &Packet) -> T + Copy,
    {
        match &self.data {
            PacketData::Literal(_) => f(state, self),
            PacketData::Operator(packets) => packets
                .iter()
                .fold(f(state, self), |acc, packet| packet.fold(acc, f)),
        }
    }

    fn fold_tree<T, F>(&self, state: T, parent: Option<&Self>, f: F) -> T
    where
        F: Fn(T, Option<&Packet>, &Packet) -> T + Copy,
    {
        match &self.data {
            PacketData::Literal(_) => f(state, parent, self),
            PacketData::Operator(packets) => {
                packets.iter().fold(f(state, parent, self), |acc, packet| {
                    packet.fold_tree(acc, Some(self), f)
                })
            }
        }
    }
}

#[allow(clippy::maybe_infinite_iter)]
fn parse(input: &BitSlice<u8, Msb0>) -> Packet {
    // dbg!(input);
    let version: u8 = input[0..3].load_be();
    let type_id: u8 = input[3..6].load_be();
    // dbg!(version, type_id);
    let mut consumed = 6;
    let data = &input[6..];
    if type_id == LITERAL {
        let mut chunks = data.chunks(5);
        let digits = chunks
            .take_while_ref(|chunk| chunk[0])
            .map(|chunk| chunk[1..].load_be::<u8>())
            .collect_vec();
        let last = chunks.next().unwrap().load_be::<u8>();
        consumed += (digits.len() + 1) * 5;
        // consumed = (consumed + 3) / 4 * 4; // Round up
        let data = digits
            .into_iter()
            .chain(iter::once(last))
            .fold(0_usize, |acc, v| acc << 4 | v as usize);
        Packet::literal(version, consumed, data)
    } else {
        // Operators
        let len_type = data[0];
        // dbg!(len_type, &data[1..]);
        if len_type == LEN_15_BITS {
            let bits: u16 = data[1..16].load_be();
            consumed += 16;
            let data = &data[16..];
            let packets = (0..)
                .scan(0usize, |counter, _| {
                    if *counter >= bits as usize {
                        None
                    } else {
                        Some({
                            let pkt = parse(&data[*counter..]);
                            *counter += pkt.size;
                            // dbg!(&pkt, counter);
                            pkt
                        })
                    }
                })
                .collect();
            consumed += bits as usize;
            // dbg!(&packets);
            Packet::operator(version, type_id, consumed, packets)
        } else {
            let packets: u16 = data[1..12].load_be();
            consumed += 12;
            // dbg!(&packets);
            let data = &data[12..];
            let packets = (0..packets)
                .scan(0usize, |counter, _| {
                    let pkt = parse(&data[*counter..]);
                    *counter += pkt.size;
                    consumed += pkt.size;
                    Some(pkt)
                })
                .collect();
            // dbg!(&packets);
            Packet::operator(version, type_id, consumed, packets)
        }
    }
}
fn preparse(input: &str) -> Vec<u8> {
    input
        .as_bytes()
        .array_chunks()
        .map(|sl @ [a, b]| from_utf8(sl).unwrap())
        .map(|sl| u8::from_str_radix(sl, 16).unwrap())
        .collect_vec()
}
pub fn part1(input: &str) -> usize {
    let packet = parse(preparse(input).view_bits::<Msb0>());
    // dbg!(&packet);
    packet.fold(0, |acc, pkt| acc + pkt.version as usize)
}

fn eval(pkt: &Packet) -> usize {
    match &pkt.data {
        &PacketData::Literal(data) => data,
        PacketData::Operator(packets) => match pkt.type_id {
            SUM => packets.iter().map(eval).sum(),
            PROD => packets.iter().map(eval).product(),
            MIN => packets.iter().map(eval).min().unwrap(),
            MAX => packets.iter().map(eval).max().unwrap(),
            LESSER => (eval(packets.first().unwrap()) < eval(packets.last().unwrap())) as usize,
            GREATER => (eval(packets.first().unwrap()) > eval(packets.last().unwrap())) as usize,
            EQUALTO => (eval(packets.first().unwrap()) == eval(packets.last().unwrap())) as usize,
            _ => panic!("Unknown operator"),
        },
    }
}

pub fn part2(input: &str) -> usize {
    let packet = parse(preparse(input).view_bits::<Msb0>());
    eval(&packet)
}

#[test]
fn test() {
    let test = "d2fe28";
    assert_eq!(
        parse(preparse(test).view_bits::<Msb0>()),
        Packet::literal(6, 21, 2021)
    );

    let test = "38006f45291200";
    assert_eq!(
        parse(preparse(test).view_bits::<Msb0>()),
        Packet::operator(
            1,
            6,
            27 + 6 + 16,
            vec![Packet::literal(6, 11, 10), Packet::literal(2, 16, 20),]
        )
    );

    let test = "ee00d40c823060";
    assert_eq!(
        parse(preparse(test).view_bits::<Msb0>()),
        Packet::operator(
            7,
            3,
            51,
            vec![
                Packet::literal(2, 11, 1),
                Packet::literal(4, 11, 2),
                Packet::literal(1, 11, 3),
            ]
        )
    );

    let test = "8A004A801A8002F478";
    assert_eq!(part1(test), 16);
    let test = "620080001611562C8802118E34";
    assert_eq!(part1(test), 12);
    let test = "C0015000016115A2E0802F182340";
    assert_eq!(part1(test), 23);
    let test = "A0016C880162017C3686B18A3D4780";
    assert_eq!(part1(test), 31);

    let test = "C200B40A82";
    assert_eq!(part2(test), 3);
    let test = "04005AC33890";
    assert_eq!(part2(test), 54);

    let input = read_input("input16.txt").unwrap();
    assert_eq!(part1(&input), 875);
    assert_eq!(part2(&input), 1264857437203);
}
