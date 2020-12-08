#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
enum ParseError {
    InvalidOperation(String),
    InvalidValue(String),
    InvalidInput(String),
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Acc(i32),
    Jmp(isize),
    Nop(isize),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(
                r"(?x)
                ^
                (?P<op>acc|jmp|nop)
                \s
                (?P<sign>[\+-])
                (?P<value>\d+)
                $
            "
            )
            .unwrap();
        }

        let capture = REGEX
            .captures(s)
            .ok_or_else(|| ParseError::InvalidInput(s.to_owned()))?;
        let operation = capture.name("op").unwrap().as_str();

        let value = capture.name("value").unwrap().as_str();
        let mut value = value
            .parse::<i32>()
            .map_err(|_| ParseError::InvalidValue(value.to_owned()))?;
        let negated = capture.name("sign").unwrap().as_str() == "-";
        if negated {
            value *= -1;
        }

        match operation {
            "acc" => Result::Ok(Instruction::Acc(value)),
            "jmp" => Result::Ok(Instruction::Jmp(value as isize)),
            "nop" => Result::Ok(Instruction::Nop(value as isize)),
            _ => Result::Err(ParseError::InvalidOperation(operation.to_owned())),
        }
    }
}

fn check(idx: usize, instructions: &Vec<Instruction>) -> Option<i32> {
    let mut pointer: usize = 0;
    let mut acc = 0;
    let mut seen: HashSet<usize> = HashSet::new();
    while !seen.contains(&pointer) {
        seen.insert(pointer);

        let mut instruction = instructions[pointer];
        if pointer == idx {
            instruction = match instruction {
                Instruction::Acc(_) => instruction,
                Instruction::Jmp(v) => Instruction::Nop(v),
                Instruction::Nop(v) => Instruction::Jmp(v),
            };
        }

        match instruction {
            Instruction::Acc(value) => {
                acc += value;
                pointer += 1;
            }
            Instruction::Jmp(value) => {
                if value.is_negative() {
                    pointer = pointer.checked_sub(value.wrapping_abs() as usize).unwrap();
                } else {
                    pointer = pointer.checked_add(value as usize).unwrap();
                }
            }
            Instruction::Nop(_) => {
                pointer += 1;
            }
        }

        if pointer >= instructions.len() {
            return Some(acc);
        }
    }

    None
}

fn main() {
    let instructions = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect::<Vec<Instruction>>();

    let mut i = 0;
    while i < instructions.len() {
        if let Some(acc) = check(i, &instructions) {
            println!("Found a valid exit point! The accumulator was {}", acc);
            break;
        }
        i += 1;
    }
}
