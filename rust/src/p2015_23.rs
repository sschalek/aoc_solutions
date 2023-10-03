// Advent of Code 2015, Day 23: "Opening the Turing Lock"
// https://adventofcode.com/2015/day/23

// Identifies a register in the emulated machine's CPU.
#[derive(Debug, Clone, Copy)]
enum RegisterId {
    A = 0,
    B = 1,
}

// Insruction set supported by the emulated CPU.
#[derive(Debug, Clone, Copy)]
enum Instruction {
    //                       ___________________________________________________________
    Hlf(RegisterId),        // |Halve       |hlf r        |r /= 2                        |
    Tpl(RegisterId),        // |Triple      |tpl r        |r *= 3                        |
    Inc(RegisterId),        // |Increment   |inc r        |r += 1                        |
    Jmp(isize),             // |Jump        |jmp offset   |ip += offset                  |
    Jie(RegisterId, isize), // |Jump if even|jie r, offset|if r % 2 == 0 { ip += offset }|
    Jio(RegisterId, isize), // |Jump if one |jio r, offset|if r == 1 { ip += offset }    |
                            // -----------------------------------------------------------
}

// Represents the emulated machine's CPU.
struct Cpu {
    registers: [usize; 2],
    ip: usize,

    is_halted: bool,

    fetch: Box<dyn Fn(usize) -> Option<Instruction>>,
}

// Provides methods for creating and interacting with a Cpu instance.
impl Cpu {
    // Creates a new Cpu instance with the given fetch function.
    // The fetch function provides the CPU with the ability to fetch an instruction
    // from memory at the given instruction pointer.
    fn new(fetch: Box<dyn Fn(usize) -> Option<Instruction>>) -> Self {
        Self {
            registers: [0; 2],
            ip: 0,
            is_halted: false,
            fetch,
        }
    }

    // Resets the CPU to its initial state.
    // If the given arguments are Some, then the registers are initialized to the given values.
    fn reset(&mut self, a: Option<usize>, b: Option<usize>) {
        self.registers[0] = a.unwrap_or(0);
        self.registers[1] = b.unwrap_or(0);
        self.ip = 0;
        self.is_halted = false;
    }

    // Tells the CPU to execute the next instruction.
    fn step(&mut self) {
        let instruction = (self.fetch)(self.ip);
        if let Some(instruction) = instruction {
            self._run_instruction(&instruction);
        } else {
            self.is_halted = true;
        }
    }

    // Private helper method that executes the given instruction.
    fn _run_instruction(&mut self, instruction: &Instruction) {
        // By default, the instruction pointer is incremented by 1.
        // Some instructions may result in a different next instruction pointer value.
        let mut next_ip = self.ip + 1;

        // Execute the given instruction.
        match instruction {
            Instruction::Hlf(register) => {
                let register = self._get_register_data_mut(*register);
                *register /= 2;
            }
            Instruction::Tpl(register) => {
                let register = self._get_register_data_mut(*register);
                *register *= 3;
            }
            Instruction::Inc(register) => {
                let register = self._get_register_data_mut(*register);
                *register += 1;
            }
            Instruction::Jmp(offset) => {
                next_ip = (self.ip as isize + offset) as usize;
            }
            Instruction::Jie(register, offset) => {
                let register = self._get_register_data_mut(*register);
                if *register % 2 == 0 {
                    next_ip = (self.ip as isize + offset) as usize;
                }
            }
            Instruction::Jio(register, offset) => {
                let register = self._get_register_data_mut(*register);
                if *register == 1 {
                    next_ip = (self.ip as isize + offset) as usize;
                }
            }
        }

        self.ip = next_ip;
    }

    // Private helper method that returns a mutable reference to the given register's data.
    fn _get_register_data_mut(&mut self, register: RegisterId) -> &mut usize {
        &mut self.registers[register as usize]
    }
}

// Implement the Debug trait for the Cpu struct, so that its state can be printed
// using the {:?} format specifier.
impl std::fmt::Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cpu")
            .field("registers", &self.registers)
            .field("ip", &self.ip)
            .field("is_halted", &self.is_halted)
            .finish_non_exhaustive()
    }
}

// Represents the emulated machine.
#[derive(Debug)]
struct Machine {
    cpu: Cpu,
}

// Provides methods for creating and interacting with a Machine instance.
impl Machine {
    // Creates a new Machine instance with the given instructions in its memory.
    fn new(instructions: Vec<Instruction>) -> Self {
        let fetch = Box::new(move |ip| instructions.get(ip).copied());
        Self { cpu: Cpu::new(fetch) }
    }

    // Runs the machine until it halts.
    fn run(&mut self) {
        while !self.cpu.is_halted {
            self.cpu.step();
        }
    }
}

// Parses the given register name into a RegisterId.
fn parse_register(register_name: &str) -> RegisterId {
    match register_name {
        "a" => RegisterId::A,
        "b" => RegisterId::B,
        _ => panic!("Invalid register name"),
    }
}

// Parses the given offset string into an isize.
fn parse_offset(offset: &str) -> isize {
    let mut chars = offset.chars();
    let sign = chars.next().unwrap();
    let value = chars.collect::<String>().parse::<isize>().unwrap();
    match sign {
        '+' => value,
        '-' => -value,
        _ => panic!("Invalid offset sign"),
    }
}

// Parses the given instruction string into an instruction of the form "<instruction_name> <register_name>".
fn parse_register_instruction<'a>(
    instruction_name: &'a str,
    arguments: &mut impl Iterator<Item = &'a str>,
) -> Instruction {
    let register = parse_register(arguments.next().unwrap());

    match instruction_name {
        "hlf" => Instruction::Hlf(register),
        "tpl" => Instruction::Tpl(register),
        "inc" => Instruction::Inc(register),
        _ => panic!("Invalid instruction name"),
    }
}

// Parses the given instruction string of the form "<instruction_name> <offset>".
fn parse_offset_instruction<'a>(
    instruction_name: &'a str,
    arguments: &mut impl Iterator<Item = &'a str>,
) -> Instruction {
    let offset = parse_offset(arguments.next().unwrap());

    match instruction_name {
        "jmp" => Instruction::Jmp(offset),
        _ => panic!("Invalid instruction name"),
    }
}

// Parses the given instruction string of the form "<instruction_name> <register_name>, <offset>".
fn parse_register_offset_instruction<'a>(
    instruction_name: &'a str,
    arguments: &mut impl Iterator<Item = &'a str>,
) -> Instruction {
    let register = parse_register(arguments.next().unwrap());
    let offset = parse_offset(arguments.next().unwrap());

    match instruction_name {
        "jie" => Instruction::Jie(register, offset),
        "jio" => Instruction::Jio(register, offset),
        _ => panic!("Invalid instruction name"),
    }
}

// Parses the given instruction string into an Instruction.
fn parse_instruction(input: &str) -> Instruction {
    // Split the instruction string into the instruction name and its arguments.
    let parts = input.split_once(' ').unwrap();
    let instruction_name = parts.0;
    let mut arguments = parts.1.split(',').map(str::trim);

    // Determine the type of instruction and parse it accordingly.
    match instruction_name {
        "hlf" | "tpl" | "inc" => parse_register_instruction(instruction_name, &mut arguments),
        "jmp" => parse_offset_instruction(instruction_name, &mut arguments),
        "jie" | "jio" => parse_register_offset_instruction(instruction_name, &mut arguments),
        _ => panic!("Invalid instruction name"),
    }
}

// Parses the given input string into a vector of Instructions.
fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.lines().map(parse_instruction).collect()
}

fn solve(input: &str, log_fn: Option<fn(&str)>) -> (String, String) {
    // Parse the input into a vector of Instructions.
    let instructions = parse_instructions(input);

    // If logging is enabled, print the parsed instructions.
    if let Some(log_fn) = log_fn {
        for (i, instruction) in instructions.iter().enumerate() {
            log_fn(&format!("{i}: {instruction:?}"));
        }
    }

    // Part 1: Run the program with register A initialized to 0.
    let mut machine = Machine::new(instructions);
    machine.run();
    let part1_result = machine.cpu.registers[1];

    // If logging is enabled, print the final state of the machine.
    if let Some(log_fn) = log_fn {
        log_fn(&format!("Part 1: {machine:?}"));
    }

    // Part 2: Run the program with register A initialized to 1.
    machine.cpu.reset(Some(1), None);
    machine.run();
    let part2_result = machine.cpu.registers[1];

    // If logging is enabled, print the final state of the machine.
    if let Some(log_fn) = log_fn {
        log_fn(&format!("Part 2: {machine:?}"));
    }

    (part1_result.to_string(), part2_result.to_string())
}

#[linkme::distributed_slice(crate::SOLUTIONS)]
static SOLUTION: crate::Solution = crate::Solution::new(2015, 23, solve);
