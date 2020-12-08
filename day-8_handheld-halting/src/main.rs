use std::time::Instant;
use std::str::FromStr;

fn main() {
    let raw_input = include_str!("../input.txt");

    let input: Vec<Instruction> = raw_input
        .lines()
        .map(|x| {
            let mut line = x.split_whitespace();
            Instruction {
                command: Command::from_str(line.next().unwrap()).unwrap(),
                value: line.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    let start = Instant::now();
    println!("Part 1 solution: {}", part_1(&input));
    println!("Part 1 was solved in: {:#?}", start.elapsed());

    let start = Instant::now();
    println!("Part 2 solution: {}", part_2(&input));
    println!("Part 2 was solved in: {:#?}", start.elapsed())

}

fn part_1(input: &Vec<Instruction>) -> isize {
    let mut acc: isize = 0;
    let mut ip = 0;

    let mut executed_list = generate_executed_list(input);

    while ip < input.len() && !executed_list[ip] {
        executed_list[ip] = true;
        execute_command(&input[ip], &mut acc, &mut ip);
    }
    acc
}

fn part_2(input: &Vec<Instruction>) -> isize {
    let mut swapped_command_pos = 0;
    for i in 0..input.len() {
        if input[i].command != Command::Acc {
            if terminator(&create_modified_input(input, i)) {
                swapped_command_pos = i;
                break;
            }
        }
    }
    return part_1(&create_modified_input(input, swapped_command_pos))
}

fn create_modified_input(input: &Vec<Instruction>, line: usize) -> Vec<Instruction> {
    let mut modified_input = input.to_vec();
    modified_input[line].command = match modified_input[line].command {
        Command::Jmp => Command::Nop,
        Command::Nop => Command::Jmp,
        y => y,
    };
    modified_input
}

fn execute_command(instruction: &Instruction, acc: &mut isize, ip: &mut usize) -> () {
    match instruction.command {
        Command::Acc => {
            *acc += instruction.value;
            *ip += 1;
        }
        Command::Jmp => {
            *ip = ((*ip as isize) + instruction.value) as usize;
        }
        _ => {
            *ip += 1;
        }
    }
}

fn terminator(input: &Vec<Instruction>) -> bool {
    let mut acc: isize = 0;
    let mut ip = 0;

    let mut executed_list = generate_executed_list(input);

    while ip < input.len() {
        if executed_list[ip] == true {
            return false;
        }
        executed_list[ip] = true;
        execute_command(&input[ip], &mut acc, &mut ip);
    }
    true
}

fn generate_executed_list(program: &Vec<Instruction>) -> Vec<bool> {
    vec![false; program.len()]
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    command: Command,
    value: isize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Command {
    Acc,
    Jmp,
    Nop,
}

impl FromStr for Command {

    type Err = ();

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        match input {
            "acc" => Ok(Command::Acc),
            "jmp" => Ok(Command::Jmp),
            "nop" => Ok(Command::Nop),
            _     => Err(()),
        }
    }
}
