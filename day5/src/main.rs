use std::fs;

fn binary_partition(line: &str, target: char, mut min_val: u32, mut max_val: u32) -> u32 {
    for c in line.chars() {
        if c == target {
            max_val = (min_val + max_val) / 2;
        } else {
            min_val = (min_val + max_val) / 2 + 1;
        }
    }
    min_val
}

fn get_row_num(line: &str) -> u32 {
    binary_partition(line, 'F', 0, 127)
}

fn get_seat_num(line: &str) -> u32 {
    binary_partition(line, 'L', 0, 7)
}

fn get_seat_id(line: &str) -> u32 {
    get_row_num(&line[..7]) * 8 + get_seat_num(&line[7..])
}

fn get_my_seat_id(seat_ids: Vec<u32>) -> u32 {
    seat_ids
        .iter()
        .enumerate()
        .find(|(idx, seat_id)| *idx > 1 && *seat_id - 2 == seat_ids[idx - 1])
        .map(|(_, seat_id)| seat_id - 1)
        .unwrap()
}

fn main() {
    let mut seat_ids = fs::read_to_string("input.txt")
        .expect("Failed to read input")
        .lines()
        .map(|line| get_seat_id(&line))
        .collect::<Vec<u32>>();

    let max_seat_id = seat_ids.iter().max().unwrap();
    println!("Max seat id: {}", max_seat_id);

    seat_ids.sort();
    let my_seat_id = get_my_seat_id(seat_ids);
    println!("My seat id is: {}", my_seat_id);
}
