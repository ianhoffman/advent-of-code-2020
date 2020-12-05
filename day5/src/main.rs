use std::fs;

fn get_seat_id(line: &str) -> u32 {
    // "line" is a string of form [F|B]{8}[L|R]{3}.
    //
    // Therefore, the last three characters encode
    // a 3-bit integer (0-7) and the first 8 characters
    // encode an 8-bit integer (0-127).
    //
    // The problem statement asks us to multiple the
    // second integer by 8 and add it to the first integer.
    //
    // Multiplying an integer by 8 is the equivalent of
    // shifting it left 3 times. Therefore, our formula
    // becomes:
    //
    // bin([F|B]{8}) << 3 + bin([L|R]{3})
    //
    // This is equivalent to:
    //
    // bin([F|B]{8}) << 3 | bin([L|R]{3})
    //
    // Which is itself equivalent to:
    //
    // bin([F|B]{8}[L|R]{3})
    //
    isize::from_str_radix(
        &line
            .replace('F', "0")
            .replace('B', "1")
            .replace('L', "0")
            .replace('R', "1"),
        2,
    ).unwrap() as u32
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
