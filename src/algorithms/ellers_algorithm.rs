use std::collections::HashMap;
use rand::Rng;
use maze::{Direction, Grid};
use super::Scan;

struct EllersHelper {
    // holds the sets for each cell in the current line
    line: Vec<usize>,
    counter: usize,
    main: usize,
    cross: usize,
    scan: Scan
}

// Based on http://weblog.jamisbuck.org/2010/12/29/maze-generation-eller-s-algorithm
// The algorithm:
// 1. Create a vector of length width | height (depends on how we scan the maze).
//      This vector will holds the set for each cell.
// 2. Run over each row (column) of the maze.
// 3. For each cell if it is closed - initialize its corresponding slot in the vector 
//      to a new set. If it is not, leave the current value in (the first row (column)
//      will always have all of its slots assigned).
// 4. Randomly join adjacent cells, but only if they are not in the same set. When joining 
//      adjacent cells, merge the cells of both sets into a single set, indicating that all 
//      cells in both sets are now connected (there is a path that connects any two cells 
//      in the set).
// 5. For each set, randomly create vertical connections downward to the next row. Each 
//      remaining set must have at least one vertical connection. The cells in the next row 
//      thus connected will share the set of the cell above them.
// 6. For the last row, join all adjacent cells that do not share a set, and omit the 
//      vertical connections.
pub fn run<R: Rng>(width: usize, height: usize, rng: &mut R, scan_dir: Scan) -> Grid {
    let mut grid = Grid::new(width, height, true);
    let mut helper = match scan_dir {
        Scan::Horizontal => EllersHelper {
            line: vec![0; width],
            counter: 0,
            main: height,
            cross: width,
            scan: Scan::Horizontal
        },
        Scan::Vertical   => EllersHelper {
            line: vec![0; height],
            counter: 0,
            main: width,
            cross: height,
            scan: Scan::Vertical
        }                            
    };
    // Run over the maze line by line 
    for i in 0 .. helper.main {
        assign_sets(&mut grid, &mut helper, i);
        carve_mainwise(&mut grid, &mut helper, i, rng);
        // Make sure not to carve out of the maze
        if i != helper.main - 1 {
            carve_crosswise(&mut grid, &helper, i, rng);
        }
    }
    grid
}

fn assign_sets(grid: &mut Grid, helper: &mut EllersHelper, i: usize) {      
    for j in 0 .. helper.cross {
        // Check how we're running over the maze 
        let is_closed = match helper.scan {
            Scan::Horizontal  => grid.cells[j][i].is_closed(),
            Scan::Vertical    => grid.cells[i][j].is_closed()
        };
        // Initialize any closed cell to a new set
        // (on the first line it will visit every cell)
        if is_closed {
            helper.line[j] = helper.counter;
            helper.counter += 1;
        }
    }
}

// Randomly merge adjacent cells from different sets
fn carve_mainwise<R: Rng>(grid: &mut Grid, helper: &mut EllersHelper, i: usize, rng: &mut R) {
    for j in 0 .. helper.cross - 1 {
        let (old, new) = (helper.line[j], helper.line[j+1]);
        // At the last line we must open every door between different sets
        if old != new && (i == helper.main - 1 || rng.gen()) {
            // merge sets
            for set in &mut helper.line {
                if *set == old { 
                    *set = new;
                }
            }
            match helper.scan {
                Scan::Horizontal  => grid.open(j, i, Direction::East),
                Scan::Vertical    => grid.open(i, j, Direction::South)
            }
        }
    }
}

fn carve_crosswise<R: Rng>(grid: &mut Grid, helper: &EllersHelper, i: usize, rng: &mut R) {
    // Order the cells by their set 
    let mut map = HashMap::new();
    for (j, &set) in helper.line.iter().enumerate() {
        let list = map.entry(set).or_insert_with(Vec::new);
        list.push(j);
    }
    // For each set choose a cell and carve south | east
    for v in map.values() {
        // Should always succeed in spite of the if let 
        if let Some(&j) = rng.choose(v) {
            match helper.scan {
                Scan::Horizontal  => grid.open(j, i, Direction::South),
                Scan::Vertical    => grid.open(i, j, Direction::East)
            }
        }
    }
}