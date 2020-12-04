use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read input.txt");

    let steps = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let product = steps.iter().fold(1, |acc, (right, down)| {
        acc * contents
            .lines()
            .enumerate()
            .filter(|(idx, _)| idx % down == 0)
            .filter(|(idx, line)| {
                line.chars()
                    .nth(((idx / down) * right) % line.len())
                    .unwrap()
                    == '#'
            })
            .count()
    });

    println!("Product: {}", product);
}
