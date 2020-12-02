use std::collections::{HashMap, HashSet};
use std::fs;

fn get_product_of_n_numbers_summing_to_target(
    nums: &HashMap<u32, usize>,
    n: u8,
    target: u32,
    exclude_indices: &mut HashSet<usize>,
) -> Option<u32> {
    if n > 0 {
        for (x, idx) in nums.clone() {
            if x > target || exclude_indices.contains(&idx) {
                continue;
            }
            // Subtract x from target and check recurse
            exclude_indices.insert(idx);
            match get_product_of_n_numbers_summing_to_target(
                &nums,
                n - 1,
                target - x,
                exclude_indices,
            ) {
                Some(product) => return Some(x * product),
                _ => {}
            }
            exclude_indices.remove(&idx);
        }
    } else if target == 0 {
        return Some(1);
    }

    return None;
}

fn main() {
    let nums = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .enumerate()
        .map(|t| (t.1.parse().unwrap(), t.0))
        .collect::<HashMap<u32, usize>>();

    match get_product_of_n_numbers_summing_to_target(&nums, 2, 2020, &mut HashSet::new()) {
        Some(product) => println!("{}", product),
        _ => {}
    }

    match get_product_of_n_numbers_summing_to_target(&nums, 3, 2020, &mut HashSet::new()) {
        Some(product) => println!("{}", product),
        _ => {}
    }
}
