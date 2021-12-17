use prelude::*;

use bitvec::prelude::*;

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    Literal(u8, u64),
    Operator(u8, Vec<Packet>),
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
        x => panic!("Unknown type {}", x),
    }
}

fn do_main(input: &str) {
    let input = hex::decode(read_lines_from_file(input).next().unwrap().as_bytes()).unwrap();
    let input = input.view_bits::<Msb0>();
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
}
