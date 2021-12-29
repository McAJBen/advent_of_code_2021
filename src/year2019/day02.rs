use crate::utils::read_input;

struct IntCodeVirtualMachine {
    memory: Vec<u32>,
    instruction_pointer: usize,
}

impl IntCodeVirtualMachine {
    fn new(input: &str) -> Self {
        let values = input
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        Self {
            memory: values,
            instruction_pointer: 0,
        }
    }

    fn get_address(&self, index: usize) -> u32 {
        self.memory[index]
    }

    fn set_address(&mut self, index: usize, value: u32) {
        self.memory[index] = value;
    }

    fn step(&mut self) {
        let operation = self.get_address(self.instruction_pointer);

        match operation {
            1 => {
                let first_parameter_index = self.get_address(self.instruction_pointer + 1) as usize;
                let second_parameter_index =
                    self.get_address(self.instruction_pointer + 2) as usize;
                let result_index = self.get_address(self.instruction_pointer + 3) as usize;

                let first_parameter = self.get_address(first_parameter_index);
                let second_parameter = self.get_address(second_parameter_index);
                let result = first_parameter + second_parameter;

                self.set_address(result_index, result);
                self.instruction_pointer += 4;
            }
            2 => {
                let first_parameter_index = self.get_address(self.instruction_pointer + 1) as usize;
                let second_parameter_index =
                    self.get_address(self.instruction_pointer + 2) as usize;
                let result_index = self.get_address(self.instruction_pointer + 3) as usize;

                let first_parameter = self.get_address(first_parameter_index);
                let second_parameter = self.get_address(second_parameter_index);
                let result = first_parameter * second_parameter;

                self.set_address(result_index, result);
                self.instruction_pointer += 4;
            }
            99 => {
                self.instruction_pointer = self.memory.len();
            }
            _ => panic!("Unknown operation: {}", operation),
        }
    }

    fn run_to_completion(&mut self) {
        while self.instruction_pointer < self.memory.len() {
            self.step();
        }
    }
}

pub fn part1() -> u32 {
    let input = read_input(2019, 2);

    let mut vm = IntCodeVirtualMachine::new(&input);

    vm.set_address(1, 12);
    vm.set_address(2, 2);

    vm.run_to_completion();

    vm.get_address(0)
}

pub fn part2() -> u32 {
    let input = read_input(2019, 2);

    for noun in 0..100 {
        for verb in 0..100 {
            let mut vm = IntCodeVirtualMachine::new(&input);
            vm.set_address(1, noun);
            vm.set_address(2, verb);
            vm.run_to_completion();
            if vm.get_address(0) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    panic!("No noun and verb found that produce 19690720");
}

#[test]
fn test_part1() {
    assert_eq!(part1(), 3765464);
}

#[test]
fn test_part2() {
    assert_eq!(part2(), 7610);
}
