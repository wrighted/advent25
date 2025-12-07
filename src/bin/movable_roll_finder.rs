use std::fs;
use std::env;

struct Spot {
    here: bool
}

impl Spot {
    fn new(here: char) -> Spot {
        Spot { here: here == '@' }
    }

    fn is_roll(&self) -> bool {
        self.here
    }
}

struct Grid {
    grid: Vec<Vec<Spot>>
}

impl Grid {
    fn num_accessible(&self) -> i32 {
        (0..self.grid.len())
        .flat_map(|i| (0..self.grid[i].len()).map(move |j| (i, j)))
        .filter(|&(i, j)| self.grid[i][j].is_roll())
        .filter(|&(i, j)| {
            [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
                .iter()
                .map(|&(di, dj)| self.is_full(i as i32 + di, j as i32 + dj))
                .sum::<i32>() < 4
        })
        .count() as i32
    }

    fn is_full(&self, i: i32, j: i32) -> i32 {
        (i >= 0 && j >= 0 
            && (i as usize) < self.grid.len() 
            && (j as usize) < self.grid[i as usize].len() 
            && self.grid[i as usize][j as usize].is_roll()) as i32
    }
}

fn main() {
    let first = env::args().nth(1).expect("please supply a filename");
    let observed = fs::read_to_string(&first).expect("can't read the file");

    let grid: Grid = Grid { 
        grid: observed.lines()
            .map(|l| l.chars().map(Spot::new).collect())
            .collect()
        };

    print!("{}", grid.num_accessible());
}
