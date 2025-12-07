use std::fs;
use std::env;

#[derive(Debug)]
struct Bank {
    batteries: [i32; Bank::SIZE]
}

impl Bank {
    const SIZE: usize = 100;

    fn new(input: &str) -> Bank {
        assert!(input.len() == Bank::SIZE);

        let bank: [i32; 100] = input.chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<_>>()
            .try_into()
            .expect("battery bank malformed");

        Bank{ batteries: bank }
    }
}

trait Joltage {
    fn max_joltage(&self, jolts: i32) -> i64;
    fn joltage_finder(batteries: &[i32], jolts: i32) -> i64;
}

impl Joltage for Bank {
    fn max_joltage(&self, jolts: i32) -> i64 {
        <Bank as Joltage>::joltage_finder(&self.batteries, jolts)
    }

    fn joltage_finder(batteries: &[i32], jolts: i32) -> i64 {
        if jolts == 0 {
            return 0
        }

        let n = batteries.len();
        let jolts_left = jolts - 1;
        
        let first = find_max_index(&batteries[..n-(jolts_left as usize)]).unwrap();

        let next_slice = first + 1;
        10_i64.pow(jolts_left as u32) * batteries[first] as i64
            + <Bank as Joltage>::joltage_finder(&batteries[next_slice..], jolts_left)
    }
}

fn find_max_index(slice: &[i32]) -> Option<usize> {
    slice.iter()
        .enumerate()
        .min_by_key(|(_, &val)| -val)
        .map(|(idx, _)| idx)
}

fn main() {
    let first = env::args().nth(1).expect("please supply a filename");
    let observed = fs::read_to_string(&first).expect("can't read the file");

    let banks: Vec<Bank> = observed.lines().map(Bank::new).collect();
    let joltage_finder = |j| banks.iter().map(|b| b.max_joltage(j)).sum();
    
    let p1: i64 = joltage_finder(2);
    let p2: i64 = joltage_finder(12);

    print!("p1: {}, p2: {}", p1, p2);
}