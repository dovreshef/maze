use rand::Rng;
use grid::Grid;
use super::{CellSelection, open_random_dir};

// Based on http://weblog.jamisbuck.org/2011/1/27/maze-generation-growing-tree-algorithm
// The algorithm:
// 1. Start with a list of cell, initialized with a random cell from the maze.
// 2. Choose a cell from the list (selection criteria given later) and open a passage
//      to an unvisited neighbor cell.
// 3. Add that neighbor to the list as well. 
// 4. If there are no unvisited neighbors, remove the cell from the list.
// 5. Repeat 2-4 until the list is empty.
// 6. Cell selection criteria can be newest cell, oldest, random, or a weighted mixture
//      of the three.
pub fn run<R: Rng>(width: usize, height: usize, rng: &mut R, selection_method: CellSelection) -> Grid {
    // Start with a raw grid
    let mut grid = Grid::new(width, height, true);
    // Choose a random starting point
    let (x, y) = (rng.gen_range(0, width), rng.gen_range(0, height));
    // Stack
    let mut cells = Vec::new();
    cells.push((x, y));
    while !cells.is_empty() {
        let (x, y, pos) = choose_cell(&cells, selection_method, rng);
        match open_random_dir(&mut grid, x, y, rng) {
            // Found a cell to move
            Some(next) => {
                cells.push(next);
            }
            // We have no unvisited cells around this cell
            None => {
                cells.remove(pos);
            }
        }
    }
    grid
}


fn choose_cell<R: Rng>(cells: &[(usize, usize)], selection_method: CellSelection, rng: &mut R) 
    -> (usize, usize, usize) {
    let np = cells.len() - 1;                           // newest
    let (nx, ny) = cells[np];
    let op = 0;                                         // oldest
    let (ox, oy) = cells[op];
    let rp = rng.gen_range(0, cells.len());            // random
    let (rx, ry) = cells[rp];
    let weight = rng.gen_range(1, 101);
    match selection_method {
        CellSelection::Newest                         => (nx, ny, np),
        CellSelection::Oldest                         => (ox, oy, op),
        CellSelection::Random                         => (rx, ry, rp),
        CellSelection::NewestOldest(p) if weight <= p => (nx, ny, np),
        CellSelection::NewestOldest(_)                => (ox, oy, op),
        CellSelection::NewestRandom(p) if weight <= p => (nx, ny, np), 
        CellSelection::NewestRandom(_)                => (rx, ry, rp),
        CellSelection::OldestRandom(p) if weight <= p => (ox, oy, op),
        CellSelection::OldestRandom(_)                => (rx, ry, rp)
    }
}