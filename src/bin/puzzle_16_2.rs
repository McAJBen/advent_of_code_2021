use std::fs::read_to_string;

fn to_num(bits: &[bool]) -> usize {
    bits.iter().fold(0, |acc, &b| (acc << 1) + (b as usize))
}

#[derive(Debug)]
enum PacketType {
    Sum { sub_packets: Vec<Packet> },
    Product { sub_packets: Vec<Packet> },
    Minimum { sub_packets: Vec<Packet> },
    Maximum { sub_packets: Vec<Packet> },
    Literal { value: u128 },
    GreaterThan { sub_packets: Vec<Packet> },
    LessThan { sub_packets: Vec<Packet> },
    EqualTo { sub_packets: Vec<Packet> },
}

#[derive(Debug)]
struct Packet {
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
        let type_id = to_num(&bits[3..6]) as u8;
        match type_id {
            4 => {
                let mut value = 0u128;
                let mut index = 6;
                loop {
                    value <<= 4;
                    value += to_num(&bits[(index + 1)..(index + 5)]) as u128;
                    if !bits[index] {
                        break;
                    }
                    index += 5;
                }
                Packet {
                    packet_type: PacketType::Literal { value },
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

                match type_id {
                    0 => Packet {
                        packet_type: PacketType::Sum { sub_packets },
                        num_bits,
                    },
                    1 => Packet {
                        packet_type: PacketType::Product { sub_packets },
                        num_bits,
                    },
                    2 => Packet {
                        packet_type: PacketType::Minimum { sub_packets },
                        num_bits,
                    },
                    3 => Packet {
                        packet_type: PacketType::Maximum { sub_packets },
                        num_bits,
                    },
                    5 => Packet {
                        packet_type: PacketType::GreaterThan { sub_packets },
                        num_bits,
                    },
                    6 => Packet {
                        packet_type: PacketType::LessThan { sub_packets },
                        num_bits,
                    },
                    7 => Packet {
                        packet_type: PacketType::EqualTo { sub_packets },
                        num_bits,
                    },
                    _ => panic!("Unknown packet type"),
                }
            }
        }
    }

    fn eval(&self) -> u128 {
        match self.packet_type {
            PacketType::Sum { ref sub_packets } => sub_packets.iter().map(Packet::eval).sum(),
            PacketType::Product { ref sub_packets } => {
                sub_packets.iter().map(Packet::eval).product()
            }
            PacketType::Minimum { ref sub_packets } => {
                sub_packets.iter().map(Packet::eval).min().unwrap()
            }
            PacketType::Maximum { ref sub_packets } => {
                sub_packets.iter().map(Packet::eval).max().unwrap()
            }
            PacketType::Literal { value } => value,
            PacketType::GreaterThan { ref sub_packets } => {
                if sub_packets[0].eval() > sub_packets[1].eval() {
                    1
                } else {
                    0
                }
            }
            PacketType::LessThan { ref sub_packets } => {
                if sub_packets[0].eval() < sub_packets[1].eval() {
                    1
                } else {
                    0
                }
            }
            PacketType::EqualTo { ref sub_packets } => {
                if sub_packets[0].eval() == sub_packets[1].eval() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

fn main() {
    let input = read_to_string("input/16").unwrap();

    let value = Packet::from_input(&input).eval();

    assert_eq!(12883091136209, value);

    println!("{}", value);
}
