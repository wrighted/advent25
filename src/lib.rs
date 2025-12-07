// Shared utilities for all problems

pub fn read_lines(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn parse_numbers<T: std::str::FromStr>(input: &str) -> Vec<T> 
where
    T::Err: std::fmt::Debug,
{
    input.lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect()
}
