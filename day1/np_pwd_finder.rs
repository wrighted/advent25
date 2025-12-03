use std::fs;
use std::env;
use std::ops::Add;

const DIAL_TICKS: i32 = 100;
const STARTING_POSITION: i32 = 50;

// The safe has a dial with only an arrow on it...
#[derive(Debug)]
struct Dial {
    tick: i32,
}

// It can point to one an only one tick...
impl Dial {
    fn new(tick: i32) -> Dial {
        Dial { tick }
    }

    fn on_zero(&self) -> bool {
        self.tick == 0
    }

    fn pass_zero(&self, twist: &Twist) -> i32 {
        let offset = if twist.negative() { DIAL_TICKS - self.tick } else { self.tick } % DIAL_TICKS;
        (offset + twist.ticks) / DIAL_TICKS
    }
}

impl Add<&Twist> for Dial {
    type Output = Dial;

    fn add(self, other: &Twist) -> Dial {
        let direction = if other.negative() { -1 } else { 1 };
        let maybe_tick = self.tick + other.ticks * direction;

        let new_position = ((maybe_tick % DIAL_TICKS) + DIAL_TICKS) % DIAL_TICKS;

        Dial { tick: new_position }
    }
}

// What would you call moving the dial?
struct Twist {
    direction: Direction,
    ticks: i32,
}

impl Twist {
    fn new(entry: &str) -> Twist {
        let direction = Direction::from_char(
            entry.chars().nth(0).expect("entry doesn't have a direction")
        );
        let ticks = entry[1..].parse::<i32>().expect("entry doesn't have a number");

        Twist { direction, ticks }
    }

    fn negative(&self) -> bool {
        self.direction == Direction::L
    }
}

// There are 2 ways you can twist the dial...
#[derive(PartialEq)]
enum Direction {
    L,
    R,
}

// The log is full of Ls and Rs...
impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'L' => Direction::L,
            'R' => Direction::R,
            _ => panic!("That's not a direction"),
        }
    }
}

fn np_decipher((dial, passes, lands): (Dial, i32, i32), twist: &Twist) -> (Dial, i32, i32) {
    let new_passes = passes + dial.pass_zero(twist);
    let new_dial = dial + twist;
    let new_lands = if new_dial.on_zero() { lands + 1 } else { lands };

    (new_dial, new_passes, new_lands)
}

// All things must start from somewhere
fn main() {
    let first = env::args().nth(1).expect("please supply a filename");
    let move_log = fs::read_to_string(&first).expect("can't read the file");

    let moves: Vec<Twist> = move_log.lines().map(Twist::new).collect();

    let result = moves.iter().fold((Dial::new(STARTING_POSITION), 0, 0), np_decipher);

    println!("The code is {:?}", result);
}
