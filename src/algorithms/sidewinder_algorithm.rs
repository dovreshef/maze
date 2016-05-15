use rand::Rng;
use grid::{Direction, Grid};
use super::Scan;

// Based on http://weblog.jamisbuck.org/2011/2/3/maze-generation-sidewinder-algorithm
// The algorithm (horizontal case):
// 1. Run over the maze by rows, for each row.
// 2. Mark the first cell as the starting position of the current set.
// 3. If we are not at the last cell in the current row, randomly decide whether to 
//          open a passage east or not.
// 4. If a passage was not opened, choose (in random) one of the cells in the current set 
//          and open a passage north. Mark the next cell as the starting point of the set.
// 5. Continue until all rows have been processed.
pub fn run<R: Rng>(width: usize, height: usize, rng: &mut R, scan_dir: Scan) -> Grid {
    let mut grid = Grid::new(width, height, true);
    let mut set_start = 0;
    // The algorithm can scan the maze Horizontally or vertically, setup
    // the run variables so they will fit.
    let (main, cross, up, side) = match scan_dir {
        Scan::Horizontal => (width, height, Direction::North, Direction::East),
        Scan::Vertical => (height, width, Direction::West, Direction::South)
    };
    for j in 0 .. cross {
        for i in 0 .. main {
            if j == 0 || rng.gen() {
                if i != main - 1 {
                    open(scan_dir, &mut grid, i, j, side);
                }
            } else {
                let idx = rng.gen_range(set_start, i + 1);
                set_start = i + 1;                   
                open(scan_dir, &mut grid, idx, j, up);
            }
        }
        set_start = 0;
    }
    grid
}

pub fn open(scan_dir: Scan, grid: &mut Grid, x: usize, y: usize, dir: Direction) {
    match scan_dir {
        Scan::Horizontal => grid.open(x, y, dir),
        Scan::Vertical   => grid.open(y, x, dir)
    }
} 
