#[macro_use]
extern crate lazy_static;

use std::fs;

fn get_directions() -> Vec<(i16, i16)> {
    lazy_static! {
        static ref DIRECTIONS: Vec<(i16, i16)> = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1)
        ];
    }
    DIRECTIONS.to_vec()
}

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn from_vec(vec: Vec<Vec<char>>) -> Grid {
        Grid { grid: vec }
    }

    fn get_pos_safe(&self, x: usize, y: usize, d1: i16, d2: i16) -> Option<(usize, usize)> {
        let x1 = x as i16 - d1;
        let y1 = y as i16 - d2;
        if x1 < 0 || y1 < 0 {
            return None;
        }
        let x1 = x1 as usize;
        let y1 = y1 as usize;
        if x1 >= self.grid.len() || y1 >= self.grid[x1].len() {
            return None;
        }
        Some((x1, y1))
    }

    fn count_occupied_neighbors(&self, x: usize, y: usize) -> u8 {
        get_directions()
            .iter()
            .fold(0, |acc, (d1, d2)| match self.get_pos_safe(x, y, *d1, *d2) {
                Some((x1, y1)) => acc + (self.grid[x1][y1] == '#') as u8,
                None => acc,
            })
    }

    fn count_occupied_seats(&self) -> u32 {
        self.grid.iter().fold(0, |a1, row| {
            a1 + row
                .iter()
                .fold(0, |a2, pos| if *pos == '#' { a2 + 1 } else { a2 })
        })
    }

    fn check_visible_occupied_seat_in_direction(
        &self,
        x: usize,
        y: usize,
        d1: i16,
        d2: i16,
    ) -> bool {
        if let Some((x1, y1)) = self.get_pos_safe(x, y, d1, d2) {
            if self.grid[x1][y1] == '#' {
                return true;
            } else if self.grid[x1][y1] == 'L' {
                return false;
            } else {
                return self.check_visible_occupied_seat_in_direction(x1, y1, d1, d2);
            }
        }
        false
    }

    fn count_visible_occupied_seats(&self, x: usize, y: usize) -> u8 {
        get_directions().iter().fold(0, |acc, (d1, d2)| {
            acc + self.check_visible_occupied_seat_in_direction(x, y, *d1, *d2) as u8
        })
    }

    fn set_pos(&mut self, x: usize, y: usize, p: char) {
        self.grid[x][y] = p;
    }

    fn apply_rule(&self, rule: fn(&Grid, usize, usize) -> u8, cutoff: u8) -> (Grid, bool) {
        let mut next_grid = self.clone();
        let mut has_changes = false;
        let mut i = 0;
        let n = self.grid.len();
        while i < n {
            let mut j = 0;
            let m = self.grid[i].len();
            while j < m {
                let pos = self.grid[i][j];
                if pos == 'L' {
                    if rule(&self, i, j) == 0 {
                        next_grid.set_pos(i, j, '#');
                        has_changes = true;
                    }
                } else if pos == '#' {
                    if rule(&self, i, j) >= cutoff {
                        next_grid.set_pos(i, j, 'L');
                        has_changes = true;
                    }
                }
                j += 1;
            }
            i += 1;
        }
        (next_grid, has_changes)
    }

    fn apply_until_complete(&self, rule: fn(&Grid, usize, usize) -> u8, cutoff: u8) -> u32 {
        let (next_grid, has_changes) = self.apply_rule(rule, cutoff);
        if !has_changes {
            return self.count_occupied_seats();
        }
        next_grid.apply_until_complete(rule, cutoff)
    }
}

fn part1(grid: &Grid) {
    let num_occupied_seats = grid.apply_until_complete(Grid::count_occupied_neighbors, 4);
    println!("Num occupied seats: {}", num_occupied_seats);
}

fn part2(grid: &Grid) {
    let num_occupied_seats = grid.apply_until_complete(Grid::count_visible_occupied_seats, 5);
    println!("Num occupied seats: {}", num_occupied_seats);
}

fn main() {
    let mut grid = vec![];
    for line in fs::read_to_string("input/input.txt").unwrap().lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }

    let grid = Grid::from_vec(grid);

    part1(&grid);
    part2(&grid);
}
