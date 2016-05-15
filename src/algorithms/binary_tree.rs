use rand::Rng;
use grid::{Direction, Grid};
use super::Bias;

// Based on http://weblog.jamisbuck.org/2011/2/1/maze-generation-binary-tree-algorithm
// The algorithm:
// 1. Iterate over the maze.
// 2. For each cell choose randomly whether to open a passage in either of two direction: 
//      (north, east), (north, west), (south, east), (south, west). 
// 3. So long as we keep to those two direction there is no danger of creating a loop
//       or leaving part of the maze inaccessible.
// Edge case: we have to be careful not to open a passage outside of the maze.
pub fn run<R: Rng>(width: usize, height: usize, rng: &mut R, bias: Bias) -> Grid {
    // Start with a raw maze
    // Run over the maze, carving in a direction chosen by choose()
    let mut grid = Grid::new(width, height, true);
    for x in 0 .. width {
        for y in 0 .. height {
            let directions = collect_directions(bias, x, y, width, height);
            if let Some(&dir) = rng.choose(&directions) {
                grid.open(x, y, dir)
            }
        }
    }
    grid
}

fn collect_directions(bias: Bias, x: usize, y: usize, width: usize, height: usize) -> Vec<Direction> {
    let mut dir = Vec::new();
    if y > 0 && (bias == Bias::Northeast || bias == Bias::Northwest) {
        dir.push(Direction::North);
    }
    if y < height - 1 && (bias == Bias::Southeast || bias == Bias::Southwest) {
        dir.push(Direction::South);
    }
    if x > 0 && (bias == Bias::Northwest || bias == Bias::Southwest) {
        dir.push(Direction::West);
    }
    if x < width - 1 && (bias == Bias::Northeast || bias == Bias::Southeast) {
        dir.push(Direction::East);
    }
    dir
}
