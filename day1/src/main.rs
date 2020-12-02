use std::collections::HashMap;
use std::env::args;
use std::fs;
use std::process;

fn get_n_numbers_summing_to_target(
    num_counts: &mut HashMap<u32, u8>,
    n: u8,
    target: u32,
) -> Option<Vec<u32>> {
    if n > 0 {
        for (x, count) in num_counts.clone() {
            if x > target || count <= 0 {
                continue;
            }
            *num_counts.get_mut(&x).unwrap() -= 1;
            match get_n_numbers_summing_to_target(num_counts, n - 1, target - x) {
                Some(mut summands) => {
                    summands.push(x);
                    return Some(summands);
                }
                _ => {}
            }
            *num_counts.get_mut(&x).unwrap() += 1;
        }
    } else if target == 0 {
        return Some(Vec::new());
    }

    return None;
}

fn main() {
    if args().count() != 3 {
        eprintln!("\nUsage: cargo run [target] [n]\n");
        process::exit(1);
    }

    let target: u32 = args()
        .nth(1)
        .unwrap()
        .parse()
        .expect("target must be a u32");
    let n: u8 = args().nth(2).unwrap().parse().expect("n must be a u8");

    let nums = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<u32>>();

    let mut num_counts: HashMap<u32, u8> = HashMap::new();
    for num in nums {
        let counter = num_counts.entry(num).or_insert(0);
        *counter += 1;
    }

    match get_n_numbers_summing_to_target(&mut num_counts, n, target) {
        Some(summands) => println!(
            "Summands: {:?}, Product: {}",
            summands,
            summands.iter().fold(1, |sum, &num| sum * num)
        ),
        _ => println!("Couldn't find {} distinct numbers summing to {}", n, target),
    }
}
