extern crate rand;
extern crate image;
extern crate rustc_serialize;
extern crate regex;

mod grid;
mod algorithms;
mod render;

use grid::Grid;
use algorithms::generate_grid;
use render::MazeRender;
pub use algorithms::{Algorithm, CellSelection, Bias, Scan};

pub struct Maze {
    width: usize,
    height: usize,
    grid: Option<Grid>    
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        if width == 0 || height == 0 {
            panic!("Maze width, height must be greater than 0.");
        }
        Maze {
            width: width,
            height: height,
            grid: None
        }
    }

    pub fn generate(&mut self, algorithm: Algorithm) -> &mut Maze {
        let grid = generate_grid(self.width, self.height, algorithm);
        self.grid = Some(grid);
        self
    }

    pub fn render(&mut self, scale: f32) -> MazeRender {
        match self.grid {
            Some(ref grid) => {
                let mut maze_render = MazeRender::new(grid);
                maze_render.scale(scale).render();
                maze_render
            }
            None => {
                panic!("The maze has not been generated yet.");
            }
        }
    }
}

#[cfg(test)]
mod tests;
