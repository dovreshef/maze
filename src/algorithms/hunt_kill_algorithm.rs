use rand::Rng;
use grid::{Direction, Grid};
use super::open_random_dir;

// Based on http://weblog.jamisbuck.org/2011/1/24/maze-generation-hunt-and-kill-algorithm
// The algorithm:
// 1. Choose a random starting position
// 2. Open a passage in a random direction to an unvisited neighbor. 
// 3. If all neighbors are visited, run over the maze line by line and find a new unvisited
//      cell that is adjacent to a visited one. 
// 4. Once found, open a passage to the visited cell from the new cell and repeat steps 2-3
// 5. If there are no more unvisited cell finish.
pub fn run<R: Rng>(width: usize, height: usize, rng: &mut R) -> Grid {
    // Start with a raw grid
    let mut grid = Grid::new(width, height, true);
    // Choose a random starting point
    let mut o = Some((rng.gen_range(0, width), rng.gen_range(0, height)));
    while let Some((x, y)) = o {
        // First case: continue where we're at
        o = open_random_dir(&mut grid, x, y, rng);
        // Second case: Hunt for a new start
        if o.is_none() {
            o = hunt(&mut grid, rng);
        }
    }
    grid
}


fn hunt<R: Rng>(grid: &mut Grid, rng: &mut R) -> Option<(usize, usize)> {
    // Run over the maze, finds any unvisited cell with visited neighbours
    let mut result = None;
    let mut directions = [Direction::North, Direction::South, Direction::East, Direction::West];
    rng.shuffle(&mut directions);        
    'l: for x in 0 .. grid.width {
        for y in 0 .. grid.height {
            // Found an closed cell
            if grid.cells[x][y].is_closed() {
                // Find an open neighbour and carve.
                // Or not, and we'll have to continue looking
                for &dir in &directions {
                    // get the position of the cell at direction dir and check that cell is unvisited
                    if let Some((nx, ny)) = grid.cell_at(x, y, dir) {
                        if !grid.cells[nx][ny].is_closed() {
                            grid.open(x, y, dir);
                            result = Some((x, y));
                            break 'l;
                        }
                    }
                }
            }
        }
    }
    result
}