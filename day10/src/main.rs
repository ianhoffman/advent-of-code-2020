use std::collections::HashMap;
use std::fs;

fn part1(adapters: &Vec<u64>) {
    let mut diffs: HashMap<u64, u64> = HashMap::new();
    let mut last = 0;
    for adapter in adapters {
        let diff = adapter - last;
        let entry = diffs.entry(diff).or_insert(0);
        *entry += 1;
        last = *adapter;
    }
    println!(
        "1-jolt diffs multiplied by 3-jolt diffs: {}",
        diffs[&1] * diffs[&3]
    );
}

fn part2(adapters: &Vec<u64>) {
    let mut num_arrangements_from_index: Vec<u64> = vec![0; adapters.len()];
    // There is just one way to arrange the final adapter.
    num_arrangements_from_index[adapters.len() - 1] = 1;

    for (i, adapter) in adapters.iter().enumerate().rev().skip(1) {
        // Look back in the `adapters` vec for adapters <= adapter + 3.
        let mut j = i + 1;
        while j < adapters.len() && adapters[j] <= adapter + 3 {
            num_arrangements_from_index[i] += num_arrangements_from_index[j];
            j += 1;
        }
    }

    println!("Distinct arrangements: {}", num_arrangements_from_index[0]);
}

fn main() {
    let mut adapters = fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    adapters.push(0);
    adapters.sort();
    // Your device's built-in adapter is always 3 higher than the highest adapter
    adapters.push(adapters.last().unwrap() + 3);

    // Part 1
    part1(&adapters);

    // Part 2
    part2(&adapters);
}
