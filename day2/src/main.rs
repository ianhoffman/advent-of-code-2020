use std::env::args;
use std::fs;
use std::process;

fn sled_rental_password_check(
    min_count: usize,
    max_count: usize,
    letter: char,
    password: String,
) -> bool {
    let count = password.chars().filter(|&ch| ch == letter).count();
    return count >= min_count && count <= max_count;
}

fn toboggan_corporate_policy_check(
    pos_a: usize,
    pos_b: usize,
    letter: char,
    password: String,
) -> bool {
    return password
        .chars()
        .enumerate()
        .filter(|t| (t.0 + 1 == pos_a || t.0 + 1 == pos_b) && letter == t.1)
        .count()
        == 1;
}

fn usage() {
    eprintln!("Usage: cargo run [strategy]");
    eprintln!(" Valid strategies:");
    eprintln!(" - toboggan");
    eprintln!(" - sled");
}

fn main() {
    if args().count() != 2 {
        usage();
        process::exit(1);
    }

    let strategy = match args().nth(1).unwrap().as_str() {
        "toboggan" => toboggan_corporate_policy_check,
        "sled" => sled_rental_password_check,
        _ => {
            usage();
            process::exit(1);
        }
    };

    let res = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt")
        .lines()
        .map(|line| line.split(" ").collect::<Vec<_>>())
        .filter(|parts| {
            let mut split = parts[0].split("-");
            return strategy(
                split.next().unwrap().to_string().parse::<usize>().unwrap(),
                split.last().unwrap().to_string().parse::<usize>().unwrap(),
                parts[1].chars().next().unwrap(),
                parts[2].to_string(),
            );
        })
        .count();

    println!("Found {} valid passwords", res);
}
