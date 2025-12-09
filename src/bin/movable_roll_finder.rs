use std::fs;
use std::env;

#[derive(Clone)]
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

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<Spot>>
}

impl Grid {
    fn num_accessible(&self) -> i32 {
        self.get_accessible().len() as i32
    }

    fn get_accessible(&self) -> Vec<(i32, i32)> {
        (0..self.grid.len())
        .flat_map(|i| (0..self.grid[i].len()).map(move |j| (i, j)))
        .filter(|&(i, j)| self.grid[i][j].is_roll())
        .filter(|&(i, j)| {
            [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
                .iter()
                .map(|&(di, dj)| self.is_full(i as i32 + di, j as i32 + dj))
                .sum::<i32>() < 4
        })
        .map(|(i, j)| (i as i32, j as i32))
        .collect()
    }

    fn is_full(&self, i: i32, j: i32) -> i32 {
        (i >= 0 && j >= 0 
            && (i as usize) < self.grid.len() 
            && (j as usize) < self.grid[i as usize].len() 
            && self.grid[i as usize][j as usize].is_roll()) as i32
    }

    fn remove_spots(&self, spots: &Vec<(i32, i32)>) -> Grid {
        let mut new_grid = self.grid.clone();
        for &(i, j) in spots {
            new_grid[i as usize][j as usize] = Spot { here: false };
        }
        Grid { grid: new_grid }
    }
}

fn elves(grid: Grid, accessed: i32) -> i32 {
    if grid.num_accessible() == 0 {
        return accessed;
    }

    let spots = grid.get_accessible();
    let new_grid = grid.remove_spots(&spots);

    elves(new_grid, accessed + spots.len() as i32)
}

fn main() {
    let first = env::args().nth(1).expect("please supply a filename");
    let observed = fs::read_to_string(&first).expect("can't read the file");

    let grid: Grid = Grid { 
        grid: observed.lines()
            .map(|l| l.chars().map(Spot::new).collect())
            .collect()
        };

    let p1 = grid.num_accessible();
    let p2 = elves(grid.clone(), 0);

    print!("p1: {}, p2: {}", p1, p2);
}
