use std::collections::HashMap;


fn main() {
    let input = vec![16, 11, 15, 0, 1, 7];
    let mut positions: HashMap<usize, usize> = input
        .iter()
        .take(input.len() - 1)
        .enumerate()
        .map(|(idx, num)| (*num, idx))
        .collect();
    let mut last_num = *input.last().unwrap();
    let mut i: usize = input.len() - 1;
    while i < 30000000 - 1 {
        let pos = match positions.get(&last_num) {
            Some(last_pos) => i - last_pos,
            None => 0
        };
        positions.insert(last_num, i);
        last_num = pos;
        i += 1;
    }
    println!("{}", last_num);
}
