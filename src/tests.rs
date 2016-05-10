use super::Maze;
use algorithms::{Algorithm, CellSelection, Bias, Scan};

const WIDTH: usize = 100;
const HEIGHT: usize = 50;
const RENDER_SCALE: f32 = 1.0;

#[test]
#[should_panic]
fn create_fail_width() {
    let _ = Maze::new(0, HEIGHT);
}

#[test]
#[should_panic]
fn create_fail_height() {
    let _ = Maze::new(WIDTH, 0);
}

#[test]
fn create_recursive_backtracking() {
    let _ = Maze::new(WIDTH, HEIGHT)
        .generate(Algorithm::RecursiveBacktracking)
        .render(RENDER_SCALE);
}

#[test]
fn create_ellers_algorithm() {
    let scan_dirs = [Scan::Horizontal, Scan::Vertical];
    for &scan in scan_dirs.iter() {
        let _ = Maze::new(WIDTH, HEIGHT)
            .generate(Algorithm::EllersAlgorithm(scan))
            .render(RENDER_SCALE);
    }
}

#[test]
fn create_kruskals_algorithm() {
    let _ = Maze::new(WIDTH, HEIGHT)
        .generate(Algorithm::KruskalsAlgorithm)
        .render(RENDER_SCALE);
}

#[test]
fn create_prims_algorithm() {
    let _ = Maze::new(WIDTH, HEIGHT)
        .generate(Algorithm::PrimsAlgorithm)
        .render(RENDER_SCALE);
}

#[test]
fn create_recursive_division() {
    let _ = Maze::new(WIDTH, HEIGHT)
        .generate(Algorithm::RecursiveDivision)
        .render(RENDER_SCALE);
}

#[test]
fn create_growing_tree() {
    let selections = [CellSelection::Newest, 
        CellSelection::Oldest, 
        CellSelection::Random, 
        CellSelection::NewestOldest(40), 
        CellSelection::NewestRandom(20), 
        CellSelection::OldestRandom(50)];
    for &selection in selections.iter() {
        let _ = Maze::new(WIDTH, HEIGHT)
            .generate(Algorithm::GrowingTree(selection))
            .render(RENDER_SCALE);
    }
}

#[test]
fn create_binary_tree() {
    let biases = [Bias::Northeast, 
        Bias::Northwest, 
        Bias::Southeast, 
        Bias::Southwest];
    for &bias in biases.iter() {
        let _ = Maze::new(WIDTH, HEIGHT)
            .generate(Algorithm::BinaryTree(bias))
            .render(RENDER_SCALE);
    }
}

#[test]
fn create_sidewinder_algorithm() {
    let scan_dirs = [Scan::Horizontal, Scan::Vertical];
    for &scan in scan_dirs.iter() {
        let _ = Maze::new(WIDTH, HEIGHT)
            .generate(Algorithm::SidewinderAlgorithm(scan))
            .render(RENDER_SCALE);
    }    
}

#[test]
fn create_hunt_kill_algorithm() {
    let _ = Maze::new(WIDTH, HEIGHT)
        .generate(Algorithm::HuntKillAlgorithm)
        .render(RENDER_SCALE);
}
