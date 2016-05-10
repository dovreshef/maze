use rand::{self, Rng};
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


pub fn generate_grid(width: usize, height: usize, algorithm: Algorithm) -> Grid {
    let mut rng = rand::thread_rng();
    let mut grid = match algorithm {
        Algorithm::BinaryTree(bias)            => binary_tree::run(width, height, &mut rng, bias),
        Algorithm::EllersAlgorithm(scan)       => ellers_algorithm::run(width, height, &mut rng, scan),
        Algorithm::RecursiveBacktracking       => recursive_backtracking::run(width, height, &mut rng),
        Algorithm::HuntKillAlgorithm           => hunt_kill_algorithm::run(width, height, &mut rng),
        Algorithm::PrimsAlgorithm              => prims_algorithm::run(width, height, &mut rng),
        Algorithm::GrowingTree(cell_selection) => growing_tree::run(width, height, &mut rng, cell_selection),
        Algorithm::SidewinderAlgorithm(scan)   => sidewinder_algorithm::run(width, height, &mut rng, scan),
        Algorithm::KruskalsAlgorithm           => kruskals_algorithm::run(width, height, &mut rng),
        Algorithm::RecursiveDivision           => recursive_division::run(width, height, &mut rng)
    };
    choose_entry_points(&mut grid, &mut rng);
    grid
}

fn choose_entry_points<R: Rng>(grid: &mut Grid, rng: &mut R) {
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

mod binary_tree;
mod ellers_algorithm;
mod recursive_backtracking;
mod hunt_kill_algorithm;
mod prims_algorithm;
mod growing_tree;
mod sidewinder_algorithm;
mod kruskals_algorithm;
mod recursive_division;