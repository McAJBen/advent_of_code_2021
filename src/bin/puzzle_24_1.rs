use std::{fs::read_to_string, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Register {
    W,
    X,
    Y,
    Z,
}

impl FromStr for Register {
    type Err = i64;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Register::W),
            "x" => Ok(Register::X),
            "y" => Ok(Register::Y),
            "z" => Ok(Register::Z),
            _ => Err(s.parse::<i64>().unwrap()),
        }
    }
}

#[derive(Debug, Clone)]
enum ALUInstruction {
    Inp(Register),
    Add(Register, Result<Register, i64>),
    Mul(Register, Result<Register, i64>),
    Div(Register, Result<Register, i64>),
    Mod(Register, Result<Register, i64>),
    Eql(Register, Result<Register, i64>),
}

#[derive(Debug, Clone)]
struct ALUProgram {
    instructions: Vec<ALUInstruction>,
    valid_inputs: Vec<Vec<i64>>,
}

impl ALUProgram {
    fn new(input: &str, valid_inputs: Vec<Vec<i64>>) -> Self {
        let instructions = input
            .lines()
            .filter_map(|line| {
                if line.starts_with("//") || line.is_empty() {
                    return None;
                }
                let pieces = line.split(' ').collect::<Vec<_>>();

                let operation = pieces[0];

                let args = pieces[1..]
                    .iter()
                    .map(|x| x.parse::<Register>())
                    .collect::<Vec<_>>();

                Some(match operation {
                    "inp" => ALUInstruction::Inp(args[0].unwrap()),
                    "add" => ALUInstruction::Add(args[0].unwrap(), args[1]),
                    "mul" => ALUInstruction::Mul(args[0].unwrap(), args[1]),
                    "div" => ALUInstruction::Div(args[0].unwrap(), args[1]),
                    "mod" => ALUInstruction::Mod(args[0].unwrap(), args[1]),
                    "eql" => ALUInstruction::Eql(args[0].unwrap(), args[1]),
                    _ => panic!("Unknown operation: {}", operation),
                })
            })
            .collect();

        ALUProgram {
            instructions,
            valid_inputs,
        }
    }
}

#[derive(Debug, Clone)]
struct Registers {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl Registers {
    fn new() -> Self {
        Registers {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    fn get_register(&self, register: Register) -> i64 {
        match register {
            Register::W => self.w,
            Register::X => self.x,
            Register::Y => self.y,
            Register::Z => self.z,
        }
    }

    fn set_register(&mut self, register: Register, value: i64) {
        match register {
            Register::W => self.w = value,
            Register::X => self.x = value,
            Register::Y => self.y = value,
            Register::Z => self.z = value,
        }
    }

    fn modify_register<F>(&mut self, register: Register, f: F)
    where
        F: FnOnce(i64) -> i64,
    {
        self.set_register(register, f(self.get_register(register)));
    }
}

#[derive(Debug, Clone)]
struct ALUProgramState<'a> {
    program: &'a ALUProgram,
    registers: Registers,
    instruction_pointer: usize,
    inputs: Vec<i64>,
}

impl<'a> ALUProgramState<'a> {
    fn new(program: &'a ALUProgram) -> Self {
        ALUProgramState {
            program,
            registers: Registers::new(),
            instruction_pointer: 0,
            inputs: Vec::new(),
        }
    }

    fn step(&self) -> Vec<Self> {
        let instruction = self.program.instructions[self.instruction_pointer].clone();

        match instruction {
            ALUInstruction::Inp(a) => {
                let valid_inputs = self.program.valid_inputs[self.inputs.len()].clone();

                valid_inputs
                    .into_iter()
                    .map(|input| {
                        let mut state = self.clone();
                        state.registers.set_register(a, input);
                        state.instruction_pointer += 1;
                        state.inputs.push(input);
                        state
                    })
                    .collect()
            }
            ALUInstruction::Add(a, b) => {
                let rhs = match b {
                    Ok(b) => self.registers.get_register(b),
                    Err(b) => b,
                };
                let mut state = self.clone();
                state.registers.modify_register(a, |x| x + rhs);
                state.instruction_pointer += 1;
                vec![state]
            }
            ALUInstruction::Mul(a, b) => {
                let rhs = match b {
                    Ok(b) => self.registers.get_register(b),
                    Err(b) => b,
                };
                let mut state = self.clone();
                state.registers.modify_register(a, |x| x * rhs);
                state.instruction_pointer += 1;
                vec![state]
            }
            ALUInstruction::Div(a, b) => {
                let rhs = match b {
                    Ok(b) => self.registers.get_register(b),
                    Err(b) => b,
                };
                let mut state = self.clone();
                state.registers.modify_register(a, |x| x / rhs);
                state.instruction_pointer += 1;
                vec![state]
            }
            ALUInstruction::Mod(a, b) => {
                let rhs = match b {
                    Ok(b) => self.registers.get_register(b),
                    Err(b) => b,
                };
                let mut state = self.clone();
                state.registers.modify_register(a, |x| x % rhs);
                state.instruction_pointer += 1;
                vec![state]
            }
            ALUInstruction::Eql(a, b) => {
                let rhs = match b {
                    Ok(b) => self.registers.get_register(b),
                    Err(b) => b,
                };
                let mut state = self.clone();
                state
                    .registers
                    .modify_register(a, |x| if x == rhs { 1 } else { 0 });
                state.instruction_pointer += 1;
                vec![state]
            }
        }
    }

    fn is_done(&self) -> bool {
        self.instruction_pointer >= self.program.instructions.len()
    }

    fn is_model_number_valid(&self) -> bool {
        self.registers.get_register(Register::Z) == 0
    }
}

fn main() {
    let input = read_to_string("input/24").unwrap();

    let program = ALUProgram::new(
        &input,
        vec![
            vec![1, 2],
            vec![9],
            vec![5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8],
            vec![8, 9],
            vec![1, 2],
            vec![2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7],
            vec![3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4],
            vec![6, 7, 8, 9],
            vec![1, 2, 3, 4, 5],
            vec![1],
            vec![8, 9],
        ],
    );

    let mut states = vec![ALUProgramState::new(&program)];

    let mut correct_input = Vec::new();

    while let Some(state) = states.pop() {
        if state.is_done() {
            if state.is_model_number_valid() {
                correct_input = state.inputs.clone();
                break;
            }
        } else {
            states.extend(state.step());
        }
    }

    let output = correct_input.into_iter().fold(0, |acc, x| acc * 10 + x);

    assert_eq!(29989297949519, output);

    println!("{}", output);
}
