use std::collections::HashSet;
use rand::Rng;
use maze::{Direction, Grid};

type Set = HashSet<(usize, usize)>;

// Based on http://weblog.jamisbuck.org/2011/1/10/maze-generation-prim-s-algorithm
// The algorithm:
// 1. Start at a random position in the maze.
// 2. Create a frontier and done sets and add the cell from 1 to the frontier set.
// 3. For each cell in the frontier set, iterate over the all neighbor cells in
//      random.
// 4. If the neighbor cell is not in the done set add it to the frontier set, and
//      open a passage to the current cell.
// 5. Remove the current cell from the frontier set and add it to the done set.
// 6. The algorithm ends when the frontier set is empty.
pub fn run<R: Rng>(width: usize, height: usize, rng: &mut R) -> Grid {
    // Start with a raw grid
    let mut grid = Grid::new(width, height, true);
    let mut done = HashSet::new();
    let mut frontier = HashSet::new();
    // Start at a random point in the maze
    let start_at = (rng.gen_range(0, width), rng.gen_range(0, height));
    frontier.insert(start_at);
    while !frontier.is_empty() {
        let (x, y) = choose(&mut frontier, rng);
        connect(&mut grid, &mut frontier, rng, &mut done, x, y);
    }
    grid
}


fn choose<R: Rng>(set: &mut Set, rng: &mut R) -> (usize, usize) {
    let idx = rng.gen_range(0, set.len());
    *set.iter().nth(idx).unwrap()
}


fn connect<R: Rng>(grid: &mut Grid, frontier: &mut Set, rng: &mut R, done: &mut Set, x: usize, y: usize) {
    let mut connected = false;
    let mut directions = [Direction::North, Direction::South, Direction::East, Direction::West];
    rng.shuffle(&mut directions);
    for &dir in &directions {
        if let Some(p) = grid.cell_at(x, y, dir) {
            // The cell doesn't belong to the done set.
            // It may already belong to the frontier set but we don't care                
            if !done.contains(&p) {
                frontier.insert(p);
            // This is the first cell that belong the done set that we've
            // found so we'll open a passage between it and the current cell                
            } else if !connected {
                grid.open(x, y, dir);
                connected = true;
            }
        }
    }
    frontier.remove(&(x, y));
    done.insert((x, y));
}