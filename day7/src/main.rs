#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::fs;

lazy_static! {
    static ref REGEX: Regex =
        Regex::new(r"^(?P<count>\d+)\s(?P<bag_type>(?:\w+\s){2})bag").unwrap();
}

struct BaggageRules {
    rules: HashMap<String, Vec<(String, u32)>>,
}

impl BaggageRules {
    fn new() -> BaggageRules {
        return BaggageRules {
            rules: HashMap::new(),
        };
    }

    fn parse_rule(&mut self, line: &str) {
        let keyword = " bags contain";
        let idx = line.find(keyword).unwrap();
        let key = line[..idx].to_owned();
        let bag_counts = line[idx + keyword.len() + 1..]
            .to_owned()
            .split(", ")
            .into_iter()
            .filter(|s| !s.ends_with("no other bags."))
            .map(|s| REGEX.captures(s).unwrap())
            .map(|c| {
                (
                    c.name("bag_type").unwrap().as_str().trim_end().to_owned(),
                    c.name("count").unwrap().as_str().parse::<u32>().unwrap(),
                )
            })
            .collect::<Vec<(String, u32)>>();

        self.rules.insert(key, bag_counts);
    }

    fn contains_type(&mut self, target: &String, current: &String) -> bool {
        match self.rules.get(current) {
            None => false,
            Some(bag_types) => bag_types
                .clone()
                .iter()
                .any(|(bag_type, _)| bag_type == target || self.contains_type(target, &bag_type)),
        }
    }

    fn get_num_containing_bag_type(&mut self, bag_type: &String) -> u32 {
        self.rules
            .clone()
            .iter()
            .fold(0, |acc, t| acc + self.contains_type(bag_type, &t.0) as u32)
    }

    fn count_bags_in_bag_type(&mut self, bag_type: &String) -> u32 {
        match self.rules.get(bag_type) {
            Some(bag_counts) if !bag_counts.is_empty() => {
                bag_counts.clone().iter().fold(0, |acc, (typ, count)| {
                    acc + count + count * self.count_bags_in_bag_type(&typ)
                })
            }
            _ => 0,
        }
    }
}

fn main() {
    let mut rules = BaggageRules::new();
    let content = fs::read_to_string("input.txt").unwrap();
    for line in content.lines() {
        rules.parse_rule(line);
    }

    let count = rules.get_num_containing_bag_type(&"shiny gold".to_string());
    println!("Num containing shiny gold: {:?}", count);

    let count = rules.count_bags_in_bag_type(&"shiny gold".to_string());
    println!("Num in shiny gold: {:?}", count);
}
