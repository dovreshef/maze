use rand::Rng;
use grid::{Direction, Grid};   

struct Line {
    fixed: usize,
    low: usize,
    high: usize,
    dir: Direction
}

struct Area {
    xs: usize,
    xe: usize,
    ys: usize,
    ye: usize
}

// Based on http://weblog.jamisbuck.org/2011/1/12/maze-generation-recursive-division-algorithm
// The algorithm:
// 1. Start with an empty grid.
// 2. Randomly draw a line either horizontally or vertically, splitting the area in two unequal
//      halves.
// 3. Randomly open a passage through the previously created line.
// 4. For each of the two resulting areas recursively repeat step 2-4.
// 5. Exit case is when the area is a single corridor.
pub fn run<R: Rng>(width: usize, height: usize, rng: &mut R) -> Grid {
    // Start with a raw grid
    let mut grid = Grid::new(width, height, false);
    // Stack of areas to divide
    let mut stack = Vec::new();
    // Start with the whole area of the maze
    stack.push(Area { xs: 0, xe: width - 1, ys: 0, ye: height - 1 });
    while let Some(a) = stack.pop() {
        if let Some((b, c)) = bisect(&mut grid, a, rng) {
            stack.push(b);
            stack.push(c);
        }
    }
    grid
}

fn bisect<R: Rng>(grid: &mut Grid, a: Area, rng: &mut R) -> Option<(Area, Area)> {
    let mut result = None;
    // Exit case: when we're down to a corridor in the maze
    if a.xe - a.xs > 1 && a.ye - a.ys > 1 {
        let (x, y) = (rng.gen_range(a.xs, a.xe), rng.gen_range(a.ys, a.ye));
        let &dir = rng.choose(&[Direction::South, Direction::East]).unwrap();
        let line;
        // Based on the direction we define the line (horizontal or vertical)
        // and the areas it delineate
        match dir {
            Direction::South => {
                line = Line { fixed: y, low: a.xs, high: a.xe, dir: Direction::South };
                result = Some((Area { ye: y, ..a }, Area { ys: y + 1, ..a }));
            }
            Direction::East => {
                line = Line { fixed: x, low: a.ys, high: a.ye, dir: Direction::East };
                result = Some((Area { xe: x, ..a }, Area { xs: x + 1, ..a }));
            }
            _ => { panic!("Unreachable fork in recursive_division::bisect") }
        };
        // Divide the area into two areas with a single passage between them
        close_line(grid, line);
        grid.open(x, y, dir);
    }
    result
}

fn close_line(grid: &mut Grid, l: Line) {
    // The line can be horizontal or vertical, based on the direction
    let mut close_passage = match l.dir {
        Direction::North | Direction::South => {
            Box::new(|i| grid.close(i, l.fixed, l.dir)) as Box<FnMut(usize) -> ()>
        }
        Direction::East | Direction::West   => {
            Box::new(|i| grid.close(l.fixed, i, l.dir)) as Box<FnMut(usize) -> ()>
        }
    };
    for i in l.low .. l.high {
        close_passage(i);
    }
}