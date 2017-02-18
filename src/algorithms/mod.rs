use rand::Rng;
use grid::{Direction, Grid};

#[derive(Clone, Copy, Debug)]
pub enum Algorithm {
    BinaryTree(Bias),
    EllersAlgorithm(Scan),
    RecursiveBacktracking,
    HuntKillAlgorithm,
    PrimsAlgorithm,
    GrowingTree(CellSelection),
    SidewinderAlgorithm(Scan),
    KruskalsAlgorithm,
    RecursiveDivision
}

#[derive(Clone, Copy, Debug)]
pub enum CellSelection {
    Newest,
    Oldest,
    Random,
    NewestOldest(usize),
    NewestRandom(usize),
    OldestRandom(usize)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Bias {
    Northeast,
    Northwest,
    Southeast,
    Southwest
}

#[derive(Clone, Copy, Debug)]
pub enum Scan {
    Horizontal,
    Vertical
}

pub fn choose_entry_points<R: Rng>(grid: &mut Grid, rng: &mut R) {
    let mut directions = [Direction::North, Direction::South, Direction::East, Direction::West];
    rng.shuffle(&mut directions);
    for &dir in directions.iter().take(2) {
        let (x, y) = match dir {
            Direction::North => (rng.gen_range(0, grid.width), 0),
            Direction::South => (rng.gen_range(0, grid.width), grid.height - 1),
            Direction::East  => (grid.width - 1, rng.gen_range(0, grid.height)),
            Direction::West  => (0, rng.gen_range(0, grid.height))
        };
        grid.open(x, y, dir);
    }
} 

 // open passage to a closed  adjacent cell in a random direction
fn open_random_dir<R: Rng>(grid: &mut Grid, x: usize, y: usize, rng: &mut R) -> Option<(usize, usize)> {
    let mut result = None;
    let mut directions = [Direction::North, Direction::South, Direction::East, Direction::West];
    rng.shuffle(&mut directions);
    for &dir in &directions {
        // get the position of the cell at direction dir and check that cell is unvisited
        if let Some((nx, ny)) = grid.cell_at(x, y, dir) {
            if grid.cells[nx][ny].is_closed() {
                grid.open(x, y, dir);
                result = Some((nx, ny));
                break;
            }
        }
    }
    result
} 

pub mod binary_tree;
pub mod ellers_algorithm;
pub mod recursive_backtracking;
pub mod hunt_kill_algorithm;
pub mod prims_algorithm;
pub mod growing_tree;
pub mod sidewinder_algorithm;
pub mod kruskals_algorithm;
pub mod recursive_division;
