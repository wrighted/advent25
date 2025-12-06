use std::fs;
use std::env;

struct Range {
    start: i64,
    stop: i64,
}

trait ProductId {
    fn is_invalid(&self, part: bool) -> bool;
    fn is_invalid_p1(&self) -> bool;
    fn is_invalid_p2(&self) -> bool;
}

impl Range {
    fn new(input_str: &str) -> Result<Range, Box<dyn std::error::Error>> {
        let (start, stop) = input_str.split_once('-')
            .ok_or("invalid range format")?;

        Ok(Range{
            start: start.trim().parse()?,
            stop: stop.trim().parse()?
        })
    }

    fn invalid_sum(&self, part: bool) -> i64 {
        (self.start..=self.stop)
            .filter(|x| x.is_invalid(part))
            .sum()
    }
}

impl ProductId for i64 {
    fn is_invalid(&self, part: bool) -> bool{
        if part { self.is_invalid_p2() } else { self.is_invalid_p1() }
    }

    fn is_invalid_p1(&self) -> bool {
        let s = self.to_string();
        let len = s.len();

        if len % 2 != 0 {
            return false;
        }

        let mid = len / 2;
        s[..mid] == s[mid..]
    }

    fn is_invalid_p2(&self) -> bool {
        let s = self.to_string();
        let len = s.len();

        let is_silly = |x: &str| x.repeat(len / x.len()) == s;

        (1..len/2+1)
            .filter(|&x| len % x == 0)
            .map(|x| &s[0..x])
            .any(is_silly)
    }
}

fn main() {
    let first = env::args().nth(1).expect("you cannot omit the filename");
    let input = fs::read_to_string(&first).expect("can't read the file");

    let ranges: Vec<Range> = input.split(',')
        .filter_map(|s| Range::new(s).ok())
        .collect();

    let silly_summer = |p| ranges.iter().map(|x| Range::invalid_sum(x, p)).sum();
    let (p1, p2): (i64, i64) = (silly_summer(false), silly_summer(true));

    println!("p1: {}, p2: {}", p1, p2);
}
