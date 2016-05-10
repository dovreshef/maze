use rand::Rng;
use grid::Grid;
use super::open_random_dir;

// Based on http://weblog.jamisbuck.org/2010/12/27/maze-generation-recursive-backtracking
// The algorithm:
// 1. Choose a random position in the maze.
// 2. Randomly choose a wall at that point and carve a passage through to the adjacent 
//      cell, but only if the adjacent cell has not been visited yet. This becomes the 
//      new current cell.
// 3. If all adjacent cells have been visited, back up to the last cell that is closed
//      and repeat step 2.
// 4. The algorithm ends when the process has backed all the way up to the starting point.
pub fn run<R: Rng>(width: usize, height: usize, rng: &mut R) -> Grid {
    // Start with a raw grid
    let mut grid = Grid::new(width, height, true);
    let mut indices = Vec::new();
    // Choose a random starting choose_entry_points
    let o = (rng.gen_range(0, width), rng.gen_range(0, height));        
    // Start with the first location (picked at random)
    indices.push(o);
    // Walk in random opening passages in the maze but only to unvisitied cells
    while !indices.is_empty() {
        let &(x, y) = indices.last().unwrap();
        match open_random_dir(&mut grid, x, y, rng) {
            // Found a cell to move
            Some(next) => {                    
                indices.push(next);
            }
            // We have no unvisited cells around this cell
            None => {
                indices.pop();
            }
        }
    }
    grid
}