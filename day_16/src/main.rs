use prelude::*;

use bitvec::prelude::*;

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    Literal(u8, u64),
    Operator(u8, u8, Vec<Packet>),
}

fn parse_packet(data: &BitSlice<Msb0, u8>) -> (Packet, &BitSlice<Msb0, u8>) {
    let version: u8 = data[0..3].load_be();
    let id: u8 = data[3..6].load_be();

    let mut remaining = &data[6..];

    match id {
        4 => {
            let mut value = 0;
            loop {
                let this: u64 = remaining[..5].load_be();
                remaining = &remaining[5..];

                value <<= 4;
                value |= this & 0xF;

                if this & 0x10 == 0 {
                    break;
                }
            }
            (Packet::Literal(version, value), remaining)
        }
        packet_type => match data[6] {
            false => {
                let payload_bits = &data[7..7 + 15].load_be::<usize>();
                let mut payload = &data[7 + 15..7 + 15 + payload_bits];
                let mut packets = Vec::new();
                while payload.len() > 0 {
                    let (packet, rest) = parse_packet(payload);
                    packets.push(packet);
                    payload = rest;
                }
                (
                    Packet::Operator(version, packet_type, packets),
                    &data[7 + 15 + payload_bits..],
                )
            }
            true => {
                let payload_count = &data[7..7 + 11].load_be::<usize>();
                let mut rest = &data[7 + 11..];
                let mut packets = Vec::new();
                for _ in 0..*payload_count {
                    let (packet, leftover) = parse_packet(rest);
                    packets.push(packet);
                    rest = leftover;
                }
                (Packet::Operator(version, packet_type, packets), rest)
            }
        },
    }
}

fn sum_packet_versions(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal(version, _) => *version as u64,
        Packet::Operator(version, _, payload) => {
            *version as u64 + payload.iter().map(sum_packet_versions).sum::<u64>()
        }
    }
}

fn packet_value(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal(_, value) => *value,
        Packet::Operator(_, packet_type, payload) => match packet_type {
            0 => payload.iter().map(packet_value).sum(),
            1 => payload.iter().map(packet_value).product(),
            2 => payload.iter().map(packet_value).min().unwrap(),
            3 => payload.iter().map(packet_value).max().unwrap(),
            5 => {
                if packet_value(&payload[0]) > packet_value(&payload[1]) {
                    1
                } else {
                    0
                }
            }
            6 => {
                if packet_value(&payload[0]) < packet_value(&payload[1]) {
                    1
                } else {
                    0
                }
            }
            7 => {
                if packet_value(&payload[0]) == packet_value(&payload[1]) {
                    1
                } else {
                    0
                }
            }
            x => panic!("unexpected packet type {}", x),
        },
    }
}

fn do_main(input: &str) {
    let input = hex::decode(read_lines_from_file(input).next().unwrap().as_bytes()).unwrap();
    let input = input.view_bits::<Msb0>();
    let (packets, padding) = parse_packet(input);

    // I assume that the input is one packet that wraps everything else, so
    assert_eq!(padding.load_be::<usize>(), 0);
    let part1 = sum_packet_versions(&packets);
    dbg!(part1);

    let part2 = packet_value(&packets);
    dbg!(part2);
}

fn main() {
    do_main("inputs/day_16.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_literal() {
        let input = b"\xD2\xFE\x28".view_bits::<Msb0>();
        let (packet, remaining) = parse_packet(input);

        assert_eq!(packet, (Packet::Literal(6, 2021)));
        assert_eq!(remaining.len(), 3);
        assert_eq!(remaining.load_be::<u64>(), 0);
    }

    #[test]
    fn parse_operator_0() {
        let input = b"\x38\x00\x6F\x45\x29\x12\x00".view_bits();
        let (packet, remaining) = parse_packet(input);

        assert_eq!(
            packet,
            Packet::Operator(1, 6, vec![Packet::Literal(6, 10), Packet::Literal(2, 20)])
        );
        assert_eq!(remaining.len(), 7);
        assert_eq!(remaining.load_be::<u64>(), 0);
    }

    #[test]
    fn parse_operator_1() {
        let input = b"\xEE\x00\xD4\x0C\x82\x30\x60".view_bits();
        let (packet, remaining) = parse_packet(input);

        assert_eq!(
            packet,
            Packet::Operator(
                7,
                3,
                vec![
                    Packet::Literal(2, 1),
                    Packet::Literal(4, 2),
                    Packet::Literal(1, 3),
                ]
            )
        );
        assert_eq!(remaining.len(), 5);
        assert_eq!(remaining.load_be::<u64>(), 0);
    }
}
