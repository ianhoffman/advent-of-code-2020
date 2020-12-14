#[macro_use]
extern crate lazy_static;

use regex::{Captures, Regex};
use std::collections::HashMap;
use std::fs;

fn mask_regex_capture(line: &str) -> Option<Captures> {
    lazy_static! {
        static ref MASK_REGEX: Regex = Regex::new(r"^mask\s=\s(?P<mask>[X01]+)$").unwrap();
    }
    MASK_REGEX.captures(line)
}

fn parse_mask_line(line: &str) -> Option<&str> {
    mask_regex_capture(line).map(|c| c.name("mask").unwrap().as_str())
}

fn mem_regex_capture(line: &str) -> Option<Captures> {
    lazy_static! {
        static ref MEM_REGEX: Regex =
            Regex::new(r"^mem\[(?P<address>\d+)\]\s=\s(?P<value>\d+)$").unwrap();
    }
    MEM_REGEX.captures(line)
}

fn parse_mem_line(line: &str) -> Option<(u64, u64)> {
    mem_regex_capture(line).map(|c| {
        (
            c.name("address").unwrap().as_str().parse::<u64>().unwrap(),
            c.name("value").unwrap().as_str().parse::<u64>().unwrap(),
        )
    })
}

fn part1(content: &str) {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut and_mask = 0;
    let mut or_mask = 0;
    for line in content.lines() {
        if let Some(mask) = parse_mask_line(line) {
            and_mask = u64::from_str_radix(mask.replace('X', "1").as_str(), 2).unwrap();
            or_mask = u64::from_str_radix(mask.replace('X', "0").as_str(), 2).unwrap();
        } else if let Some((address, value)) = parse_mem_line(line) {
            mem.insert(address, and_mask & (or_mask | value));
        } else {
            panic!("Unmatched input {}!", line);
        }
    }
    let sum: u64 = mem.values().sum();
    println!("Sum: {}", sum);
}

fn gen_v2_addresses(address: u64, mask: &str, i: usize) -> Vec<u64> {
    if i >= mask.len() {
        return vec![0];
    }
    let curr = mask.chars().nth(mask.len() - i - 1).unwrap();
    gen_v2_addresses(address, mask, i + 1)
        .iter()
        .map(|&n| {
            if curr == 'X' {
                vec![n, n | (1 << i)]
            } else if curr == '1' {
                vec![n | 1 << i]
            } else {
                vec![n | (address & (1 << i))]
            }
        })
        .flatten()
        .collect()
}

fn part2(content: &str) {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask = "";
    for line in content.lines() {
        if let Some(m) = parse_mask_line(line) {
            mask = m;
        } else if let Some((address, value)) = parse_mem_line(line) {
            for addr in gen_v2_addresses(address, mask, 0) {
                mem.insert(addr, value);
            }
        }
    }
    let sum: u64 = mem.values().sum();
    println!("Sum: {}", sum);
}

fn main() {
    let content = fs::read_to_string("input/input.txt").unwrap();
    part1(&content);
    part2(&content);
}
