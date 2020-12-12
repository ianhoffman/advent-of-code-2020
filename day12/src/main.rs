use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Waypoint {
    north: i32,
    east: i32,
}

#[derive(Debug)]
struct Ship {
    north: i32,
    east: i32,
    angle: i32,
    waypoint: Waypoint,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            north: 0,
            east: 0,
            angle: 0, // Facing east
            waypoint: Waypoint { north: 1, east: 10 },
        }
    }

    fn apply_action(&mut self, action: &Action) {
        match action {
            Action::MoveY(amount) => self.north += amount,
            Action::MoveX(amount) => self.east += amount,
            Action::Turn(amount) => self.angle = (self.angle + amount) % 360,
            Action::MoveForward(amount) => {
                if self.angle == 0 {
                    self.east += amount;
                } else if self.angle == 180 {
                    self.east -= amount;
                } else if self.angle == 90 {
                    self.north += amount;
                } else if self.angle == 270 {
                    self.north -= amount;
                } else {
                    panic!("Invaid angle {}", self.angle);
                }
            }
        }
    }

    fn apply_action_waypoint(&mut self, action: &Action) {
        match action {
            Action::MoveY(amount) => self.waypoint.north += amount,
            Action::MoveX(amount) => self.waypoint.east += amount,
            Action::Turn(amount) => {
                let amount = *amount;
                if amount == 90 {
                    // Clockwise rotation 90 deg
                    let temp = self.waypoint.north;
                    self.waypoint.north = self.waypoint.east * -1;
                    self.waypoint.east = temp;
                } else if amount == 180 {
                    // Flip
                    self.waypoint.north *= -1;
                    self.waypoint.east *= -1;
                } else if amount == 270 {
                    // Clockwise rotation 270 deg
                    let temp = self.waypoint.north;
                    self.waypoint.north = self.waypoint.east;
                    self.waypoint.east = temp * -1;
                } else if amount != 0 {
                    panic!("Invalid amount {}", amount);
                }
            }
            Action::MoveForward(amount) => {
                self.north += amount * self.waypoint.north;
                self.east += amount * self.waypoint.east;
            }
        }
    }

    fn manhattan_distance(&self) -> i32 {
        return self.north.abs() + self.east.abs();
    }
}

#[derive(Debug)]
struct ActionParseError(String);

#[derive(Debug)]
enum Action {
    MoveY(i32),
    MoveX(i32),
    Turn(i32),
    MoveForward(i32),
}

impl FromStr for Action {
    type Err = ActionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code = &s[..1];
        let value = &s[1..].parse::<i32>().unwrap();
        match code {
            "N" => Ok(Action::MoveY(*value)),
            "S" => Ok(Action::MoveY(*value * -1)),
            "E" => Ok(Action::MoveX(*value)),
            "W" => Ok(Action::MoveX(*value * -1)),
            "L" => Ok(Action::Turn(360 - (*value % 360))),
            "R" => Ok(Action::Turn(*value % 360)),
            "F" => Ok(Action::MoveForward(*value)),
            _ => Err(ActionParseError(s.to_owned())),
        }
    }
}

fn part1(actions: &Vec<Action>) {
    let mut ship = Ship::new();
    for action in actions {
        ship.apply_action(action);
    }
    println!("Part 1: {}", ship.manhattan_distance());
}

fn part2(actions: &Vec<Action>) {
    let mut ship = Ship::new();
    for action in actions {
        ship.apply_action_waypoint(action);
    }
    println!("Part 2: {}", ship.manhattan_distance());
}

fn main() {
    let actions: Vec<Action> = fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<Action>().unwrap())
        .collect();
    part1(&actions);
    part2(&actions);
}
