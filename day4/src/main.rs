use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs;
use std::result::Result;
use std::str::FromStr;

#[derive(Debug)]
struct PassportError {
    message: String,
}

impl Error for PassportError {}

impl Display for PassportError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "PassportError({})", self.message)
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum PassportFieldType {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportID,
    CountryID,
}

impl FromStr for PassportFieldType {
    type Err = PassportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "byr" => Result::Ok(PassportFieldType::BirthYear),
            "iyr" => Result::Ok(PassportFieldType::IssueYear),
            "eyr" => Result::Ok(PassportFieldType::ExpirationYear),
            "hgt" => Result::Ok(PassportFieldType::Height),
            "hcl" => Result::Ok(PassportFieldType::HairColor),
            "ecl" => Result::Ok(PassportFieldType::EyeColor),
            "pid" => Result::Ok(PassportFieldType::PassportID),
            "cid" => Result::Ok(PassportFieldType::CountryID),
            _ => Result::Err(PassportError {
                message: format!("Invalid field type: {}", s),
            }),
        }
    }
}

impl PassportFieldType {
    fn check_in_range(data: &String, start: u16, end: u16) -> bool {
        let value = data.parse::<u16>().unwrap_or(0);
        value >= start && value <= end
    }

    pub fn is_valid(&self, value: String) -> bool {
        match self {
            PassportFieldType::BirthYear => PassportFieldType::check_in_range(&value, 1920, 2002),
            PassportFieldType::IssueYear => PassportFieldType::check_in_range(&value, 2010, 2020),
            PassportFieldType::ExpirationYear => {
                PassportFieldType::check_in_range(&value, 2020, 2030)
            }
            PassportFieldType::Height => {
                let mut amount = String::new();
                let mut iter = value.chars().peekable();
                while iter.peek().unwrap_or(&' ').is_numeric() {
                    amount.push(iter.next().unwrap());
                }
                match iter.collect::<String>().as_str() {
                    "cm" => PassportFieldType::check_in_range(&amount, 150, 193),
                    "in" => PassportFieldType::check_in_range(&amount, 59, 76),
                    _ => false,
                }
            }
            PassportFieldType::HairColor => {
                let mut chars = value.chars();
                // Require 7 chars total:
                // first is #, and the rest are per the pattern below.
                if chars.next().unwrap_or(' ') != '#' {
                    return false;
                }
                chars
                    .filter(|ch| match ch {
                        '0'..='9' | 'a'..='f' => true,
                        _ => false,
                    })
                    .count()
                    == 6
            }
            PassportFieldType::EyeColor => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .iter()
                .cloned()
                .collect::<HashSet<&str>>()
                .contains(value.as_str()),
            PassportFieldType::PassportID => value.chars().filter(|c| c.is_numeric()).count() == 9,
            PassportFieldType::CountryID => false,
        }
    }
}

struct Passport {
    fields: HashMap<PassportFieldType, String>,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            fields: HashMap::new(),
        }
    }

    fn add_field(&mut self, field_type: PassportFieldType, value: String) {
        if !self.fields.contains_key(&field_type) {
            self.fields.insert(field_type, value);
        }
    }

    fn is_empty(&self) -> bool {
        return self.fields.is_empty();
    }

    fn is_valid(&self) -> bool {
        // Look for 7 valid fields: one for each PassportFieldType,
        // excluding CountryID since it's optional.
        self.fields
            .iter()
            .filter(|(field, value)| field.is_valid(value.to_string()))
            .count()
            == 7
    }
}

fn get_passports(content: &String) -> Vec<Passport> {
    let mut passports = Vec::new();
    let mut current_passport = Passport::new();
    for line in content.lines() {
        if line.is_empty() {
            passports.push(current_passport);
            current_passport = Passport::new();
        } else {
            for kv in line.split(" ") {
                let mut parts = kv.split(":");
                match parts.next().unwrap_or("").parse::<PassportFieldType>() {
                    Result::Ok(field_type) => {
                        if let Some(value) = parts.next() {
                            current_passport.add_field(field_type, value.to_string());
                        }
                    }
                    Result::Err(_) => continue,
                }
            }
        }
    }
    if !current_passport.is_empty() {
        passports.push(current_passport);
    }
    passports
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let num_valid = get_passports(&content)
        .iter()
        .filter(|passport| passport.is_valid())
        .count();
    println!("Found {} valid passports", num_valid);
}
