use std::cmp;
use std::collections::{vec_deque, HashMap, VecDeque};
use std::fs;
use std::hash::Hash;

struct OrderedSet<T> {
    counts: HashMap<T, u32>,
    values: VecDeque<T>,
}

impl<T> OrderedSet<T>
where
    T: Copy + Eq + Hash,
{
    fn new() -> Self {
        OrderedSet {
            counts: HashMap::new(),
            values: VecDeque::new(),
        }
    }

    fn push(&mut self, value: T) {
        self.values.push_back(value);
        let entry = self.counts.entry(value).or_insert(0);
        *entry += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if let Some(value) = self.values.pop_front() {
            if let Some(count) = self.counts.get_mut(&value) {
                *count -= 1;
            }
            return Some(value);
        }
        None
    }

    fn contains(&self, value: T) -> bool {
        self.counts.contains_key(&value)
    }

    fn values(&self) -> vec_deque::Iter<T> {
        self.values.iter()
    }
}

fn find_invalid_num(set: &mut OrderedSet<u64>, nums: &Vec<u64>) -> Option<u64> {
    for num in nums.iter().skip(25) {
        let is_valid = set
            .values()
            .filter(|v| *v <= num)
            .find(|v| {
                let target = *num - *v;
                target != **v && set.contains(target)
            })
            .is_some();
        if !is_valid {
            return Some(*num);
        }
        set.push(*num);
        set.pop();
    }
    None
}

fn find_sum(nums: &Vec<u64>, target: u64) -> Option<u64> {
    let mut start = 0;
    let mut end = 0;
    let mut sum = nums[end];
    while start <= end {
        if sum < target {
            end += 1;
            sum += nums[end];
        } else if sum > target {
            sum -= nums[start];
            start += 1;
        } else {
            let min = &nums[start..end].iter().min().unwrap();
            let max = &nums[start..end].iter().max().unwrap();
            return Some(*min + *max);
        }
    }
    None
}

fn find_sum_brute_force(nums: &Vec<u64>, target: u64) -> Option<u64> {
    let mut i = 0;
    while i < nums.len() {
        let mut min = u64::MAX;
        let mut max = u64::MIN;
        let mut sum = 0;
        let mut j = i;
        while j < nums.len() && sum < target {
            sum += nums[j];
            min = cmp::min(min, nums[j]);
            max = cmp::max(max, nums[j]);
            j += 1;
        }
        if sum == target {
            return Some(min + max);
        }
        i += 1;
    }
    None
}

fn main() {
    let mut set: OrderedSet<u64> = OrderedSet::new();
    let nums = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    for num in nums.iter().take(25) {
        set.push(*num);
    }

    if let Some(invalid_num) = find_invalid_num(&mut set, &nums) {
        println!("Invalid num: {}", invalid_num);

        if let Some(sum) = find_sum_brute_force(&nums, invalid_num) {
            println!("Target sum: {}", sum);
        }
        if let Some(sum) = find_sum(&nums, invalid_num) {
            println!("Target sum: {}", sum);
        }
    } else {
        println!("Couldn't find an invalid num");
    }
}
