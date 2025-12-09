use std::fs;
use std::env;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct IdRange {
    start: i64,
    end: i64,
}

impl IdRange {
    fn new(input: &str) -> IdRange {
        let parts: Vec<&str> = input.split("-").collect();
        IdRange {
            start: parts[0].parse().unwrap(),
            end: parts[1].parse().unwrap(),
        }
    }

    fn contains(&self, id: i64) -> bool {
        id >= self.start && id <= self.end
    }
}

fn fresh_counter(range: &[IdRange], i: i64 , acc: i64) -> i64 {
    if range.is_empty() {
        return acc;
    }

    let start = i.max(range[0].start);

    if range[0].end < start {
        return fresh_counter(&range[1..], start, acc)
    }

    let num = range[0].end - start + 1;
    let j = range[0].end + 1;

    fresh_counter(&range[1..], j, acc + num)
}

fn main() {
    let first = env::args().nth(1).expect("please supply a filename");
    let observed = fs::read_to_string(&first).expect("can't read the file");

    let parts = observed.split("\n\n").collect::<Vec<&str>>();
    let mut fresh_ids: Vec<IdRange> = parts[0].split("\n").map(IdRange::new).collect();

    let ids: Vec<&str> = parts[1].split("\n").collect();

    let p1 = ids.iter()
        .filter(|id_str| {
            let id: i64 = id_str.parse().unwrap();
            fresh_ids.iter().any(|range| range.contains(id))
        })
        .count();

    fresh_ids.sort();
    let p2: i64 = fresh_counter(&fresh_ids, 0, 0);
    
    println!("p1: {}", p1);
    println!("p2: {}", p2);
}