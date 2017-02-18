extern crate rand;
extern crate image;

mod grid;
mod algorithms;
mod render;

use algorithms::binary_tree;
use algorithms::ellers_algorithm;
use algorithms::recursive_backtracking;
use algorithms::hunt_kill_algorithm;
use algorithms::prims_algorithm;
use algorithms::growing_tree;
use algorithms::sidewinder_algorithm;
use algorithms::kruskals_algorithm;
use algorithms::recursive_division;
use algorithms::choose_entry_points;
use grid::Grid;
use render::MazeRender;

pub use algorithms::{Algorithm, CellSelection, Bias, Scan};

pub struct Maze {
    width: usize,
    height: usize,
    algorithm: Algorithm 
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        if width == 0 || height == 0 {
            panic!("Maze width, height must be greater than 0.");
        }
        Maze {
            width: width,
            height: height,
            algorithm: Algorithm::RecursiveBacktracking
        }
    }

    pub fn algorithm(&mut self, algorithm: Algorithm) -> &mut Maze {
        self.algorithm = algorithm;
        self
    }

    fn generate_grid(&mut self) -> Grid {
        let mut rng = rand::thread_rng();
        let width = self.width;
        let height = self.height;
        let mut grid = match self.algorithm {
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

    pub fn render(&mut self, scale: f32) -> MazeRender {
        let mut grid = self.generate_grid();
        let mut maze_render = MazeRender::new(&mut grid);
        maze_render.scale(scale).render();
        maze_render
    }
}

#[cfg(test)]
mod tests;
