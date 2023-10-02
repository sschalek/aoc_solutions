// General purpose CPU registers.
#[derive(Debug, Clone, Copy)]
enum Register {
    A,
    B,
}

// Insruction set supported by the CPU.
#[derive(Debug, Clone, Copy)]
enum Instruction {
    Hlf(Register),        // Halve -        hlf r; r /= 2
    Tpl(Register),        // Triple -       tpl r; r *= 3
    Inc(Register),        // Increment -    inc r; r += 1
    Jmp(isize),           // Jump -         jmp offset; ip += offset
    Jie(Register, isize), // Jump if even - jie r, offset; if r % 2 == 0 { ip += offset }
    Jio(Register, isize), // Jump if one -  jio r, offset; if r == 1 { ip += offset }
}

struct Cpu {
    registers: [usize; 2],
    ip: usize,

    is_halted: bool,

    fetch: Box<dyn Fn(usize) -> Option<Instruction>>,
}

impl std::fmt::Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cpu")
            .field("registers", &self.registers)
            .field("ip", &self.ip)
            .field("is_halted", &self.is_halted)
            .finish_non_exhaustive()
    }
}

impl Cpu {
    fn new(fetch: Box<dyn Fn(usize) -> Option<Instruction>>) -> Self {
        Self {
            registers: [0; 2],
            ip: 0,
            is_halted: false,
            fetch,
        }
    }

    fn reset(&mut self, a: Option<usize>, b: Option<usize>) {
        self.registers[0] = a.unwrap_or(0);
        self.registers[1] = b.unwrap_or(0);
        self.ip = 0;
        self.is_halted = false;
    }

    fn step(&mut self) {
        let instruction = (self.fetch)(self.ip);
        if let Some(instruction) = instruction {
            self.run_instruction(&instruction);
        } else {
            self.is_halted = true;
        }
    }

    fn run_instruction(&mut self, instruction: &Instruction) {
        let mut next_ip = self.ip + 1;

        match instruction {
            Instruction::Hlf(register) => {
                let register = self.get_register_mut(*register);
                *register /= 2;
            }
            Instruction::Tpl(register) => {
                let register = self.get_register_mut(*register);
                *register *= 3;
            }
            Instruction::Inc(register) => {
                let register = self.get_register_mut(*register);
                *register += 1;
            }
            Instruction::Jmp(offset) => {
                next_ip = (self.ip as isize + offset) as usize;
            }
            Instruction::Jie(register, offset) => {
                let register = self.get_register_mut(*register);
                if *register % 2 == 0 {
                    next_ip = (self.ip as isize + offset) as usize;
                }
            }
            Instruction::Jio(register, offset) => {
                let register = self.get_register_mut(*register);
                if *register == 1 {
                    next_ip = (self.ip as isize + offset) as usize;
                }
            }
        }

        self.ip = next_ip;
    }

    // Returns a mutable reference to the register specified by the given register enum value.
    fn get_register_mut(&mut self, register: Register) -> &mut usize {
        match register {
            Register::A => &mut self.registers[0],
            Register::B => &mut self.registers[1],
        }
    }
}

#[derive(Debug)]
struct Machine {
    cpu: Cpu,
}

impl Machine {
    fn new(instructions: Vec<Instruction>) -> Self {
        let fetch = Box::new(move |ip| instructions.get(ip).copied());
        Self { cpu: Cpu::new(fetch) }
    }

    fn run(&mut self) {
        while !self.cpu.is_halted {
            self.cpu.step();
        }
    }
}

fn parse_register(register_name: &str) -> Register {
    match register_name {
        "a" => Register::A,
        "b" => Register::B,
        _ => panic!("Invalid register name"),
    }
}

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

fn parse_instruction(input: &str) -> Instruction {
    let parts = input.split_once(' ').unwrap();
    let instruction_name = parts.0;
    let mut arguments = parts.1.split(',').map(str::trim);
    match instruction_name {
        "hlf" | "tpl" | "inc" => parse_register_instruction(instruction_name, &mut arguments),
        "jmp" => parse_offset_instruction(instruction_name, &mut arguments),
        "jie" | "jio" => parse_register_offset_instruction(instruction_name, &mut arguments),
        _ => panic!("Invalid instruction name"),
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.lines().map(parse_instruction).collect()
}

fn solve(input: &str, log_fn: Option<fn(&str)>) -> (String, String) {
    let instructions = parse_instructions(input);
    if let Some(log_fn) = log_fn {
        for (i, instruction) in instructions.iter().enumerate() {
            log_fn(&format!("{i}: {instruction:?}"));
        }
    }

    // Part 1: Run the program with register A initialized to 0.
    let mut machine = Machine::new(instructions);
    machine.run();
    if let Some(log_fn) = log_fn {
        log_fn(&format!("Part 1: {machine:?}"));
    }
    let part1_result = machine.cpu.registers[1];

    // Part 2: Run the program with register A initialized to 1.
    machine.cpu.reset(Some(1), None);
    machine.run();
    if let Some(log_fn) = log_fn {
        log_fn(&format!("Part 2: {machine:?}"));
    }
    let part2_result = machine.cpu.registers[1];

    (part1_result.to_string(), part2_result.to_string())
}

#[linkme::distributed_slice(crate::SOLUTIONS)]
static SOLUTION: crate::Solution = crate::Solution::new(2015, 23, solve);
