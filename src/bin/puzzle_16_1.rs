use std::fs::read_to_string;

fn to_num(bits: &[bool]) -> usize {
    bits.iter().fold(0, |acc, &b| (acc << 1) + (b as usize))
}

#[derive(Debug)]
enum PacketType {
    Literal,
    Operator { sub_packets: Vec<Packet> },
}

#[derive(Debug)]
struct Packet {
    version: u8,
    packet_type: PacketType,
    num_bits: usize,
}

impl Packet {
    fn from_input(input: &str) -> Self {
        let bits = input
            .chars()
            .flat_map(|char| {
                let bits = char.to_digit(16).unwrap();

                (0..4)
                    .rev()
                    .map(|i| (bits >> i) & 1 == 1)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Packet::from_bits(&bits)
    }

    fn from_bits(bits: &[bool]) -> Self {
        let version = to_num(&bits[0..3]) as u8;
        let type_id = to_num(&bits[3..6]) as u8;
        match type_id {
            4 => {
                let mut index = 6;
                loop {
                    if !bits[index] {
                        break;
                    }
                    index += 5;
                }
                Packet {
                    version,
                    packet_type: PacketType::Literal,
                    num_bits: index + 5,
                }
            }
            _ => {
                let length_type_id = bits[6];
                let mut sub_packets = Vec::new();

                let mut num_bits;
                if length_type_id {
                    let num_sub_packets = to_num(&bits[7..18]);
                    num_bits = 18;
                    for _ in 0..num_sub_packets {
                        let packet = Packet::from_bits(&bits[num_bits..]);
                        num_bits += packet.num_bits;
                        sub_packets.push(packet);
                    }
                } else {
                    let last_bit = to_num(&bits[7..22]) + 22;
                    num_bits = 22;
                    while num_bits < last_bit {
                        let packet = Packet::from_bits(&bits[num_bits..]);
                        num_bits += packet.num_bits;
                        sub_packets.push(packet);
                    }
                }
                Packet {
                    version,
                    packet_type: PacketType::Operator { sub_packets },
                    num_bits,
                }
            }
        }
    }

    fn version_sum(&self) -> u128 {
        match self.packet_type {
            PacketType::Literal => self.version as u128,
            PacketType::Operator { ref sub_packets } => {
                sub_packets
                    .iter()
                    .map(|packet| packet.version_sum())
                    .sum::<u128>()
                    + self.version as u128
            }
        }
    }
}

fn main() {
    let input = read_to_string("puzzle_16_input").unwrap();

    let version_sum = Packet::from_input(&input).version_sum();

    assert_eq!(967, version_sum);

    println!("{}", version_sum);
}
