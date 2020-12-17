use std::collections::HashMap;
use std::fs;

type Position = (i32, i32, i32, i32);

#[derive(Debug)]
struct Universe {
    cubes: HashMap<Position, bool>,
    planar_width: u32,
    depth: u32,
}

impl Universe {
    fn get_num_active_neighbors(&self, x: i32, y: i32, z: i32, w: i32) -> u32 {
        let mut num_active = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    for dw in -1..=1 {
                        if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                            continue;
                        }

                        if *self.is_active(&(x + dx, y + dy, z + dz, w + dw)) {
                            num_active += 1;
                        }
                    }
                }
            }
        }
        num_active
    }

    fn get_num_active(&self) -> u32 {
        self.cubes
            .keys()
            .fold(0, |acc, k| acc + *self.is_active(k) as u32)
    }

    fn is_active(&self, pos: &Position) -> &bool {
        self.cubes.get(pos).unwrap_or(&false)
    }

    fn pprint(&self) {
        let width = self.planar_width as i32;
        let depth = self.depth as i32;
        for w in (depth * -1)..=depth {
            for z in (depth * -1)..=depth {
                println!("z={}, w={}", z, w);
                for x in (width * -1)..=width {
                    for y in (width * -1)..=width {
                        if *self.is_active(&(x, y, z, w)) {
                            print!("# ");
                        } else {
                            print!(". ");
                        }
                    }
                    print!("\n");
                }
                print!("\n");
            }
        }
    }

    fn next_state(&self) -> Universe {
        let next_width: i32 = self.planar_width as i32 + 1;
        let next_depth: i32 = self.depth as i32 + 1;
        let cubes = ((next_width * -1)..=next_width)
            .map(|x| {
                ((next_width * -1)..=next_width)
                    .map(move |y| {
                        ((next_depth * -1)..=next_depth)
                            .map(move |z| {
                                ((next_depth * -1)..=next_depth).map(move |w| {
                                    let num_active_neighbors =
                                        self.get_num_active_neighbors(x, y, z, w);
                                    let is_active = match self.is_active(&(x, y, z, w)) {
                                        true => {
                                            num_active_neighbors == 2 || num_active_neighbors == 3
                                        }
                                        false => num_active_neighbors == 3,
                                    };
                                    ((x, y, z, w), is_active)
                                })
                            })
                            .flatten()
                    })
                    .flatten()
            })
            .flatten()
            .collect::<HashMap<(i32, i32, i32, i32), bool>>();

        Universe {
            cubes: cubes,
            planar_width: next_width as u32,
            depth: next_depth as u32,
        }
    }
}

fn main() {
    let content = fs::read_to_string("../input/input.txt").unwrap();

    let cubes = content
        .lines()
        .enumerate()
        .map(|(x, chars)| {
            chars.split("").enumerate().map(move |(y, chr)| {
                let width = chars.len() as i32 / 2;
                (((x as i32) - width, (y as i32) - width, 0, 0), chr == "#")
            })
        })
        .flatten()
        .collect::<HashMap<(i32, i32, i32, i32), bool>>();

    let width = content.split("\n").next().unwrap().len() as u32 / 2;
    let mut universe = Universe {
        cubes: cubes,
        planar_width: width,
        depth: 0,
    };
    for _ in 0..6 {
        universe = universe.next_state();
    }

    println!("Num active: {}", universe.get_num_active());
}
