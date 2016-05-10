use rand::Rng;
use maze::{Direction, Grid};

struct Sets {
    width: usize,
    // each cell holds the location of its parent or itself if it is root
    cells: Vec<usize>
}

impl Sets {
    fn new(width: usize, height: usize) -> Sets {
        let size = width * height;
        Sets {
            width: width,
            cells: (0..size).map(|i| i).collect()
        }
    }

    fn root(&mut self, x: usize, y: usize) -> usize {
        let mut l = y * self.width + x;
        while self.cells[l] != l {
            // flatten the tree some
            self.cells[l] = self.cells[self.cells[l]];
            // move up the tree
            l = self.cells[l];
        }
        l
    }

    // attempt to join the two sets if they're not joined already
    fn reparent(&mut self, fx: usize, fy: usize, sx: usize, sy: usize) -> bool {
        let mut result = false;
        let parent = self.root(fx, fy);
        let child = self.root(sx, sy);
        if parent != child {
            self.cells[child] = parent;
            result = true;
        }
        result
    }
}

type Edge = (usize, usize, Direction);

// Based on http://weblog.jamisbuck.org/2011/1/3/maze-generation-kruskal-s-algorithm
// The algorithm:
// 1. Collect all unique edges in the maze into a set of sets.
// 2. Sort all the edges in random order.
// 3. Iterate over the set. For each edge, if the edge connects two disjoint set,
//      join the sets and open a passage between them. otherwise, do nothing.
pub fn run<R: Rng>(width: usize, height: usize, rng: &mut R) -> Grid {
    // Start with a raw grid
    let mut grid = Grid::new(width, height, true);
    let mut sets = Sets::new(width, height);
    let mut edges = collect_edges(width, height);
    rng.shuffle(&mut edges);
    // Run over all the edges in the maze
    for &(x, y, dir) in &edges {
        // This should never fail
        let (nx, ny) = grid.cell_at(x, y, dir).unwrap();
        // If the two adjacent cells don't belong to the same set
        if sets.reparent(x, y, nx, ny) {
            // Remove the edge between them
            grid.open(x, y, dir);
        }
    }
    grid
}

fn collect_edges(width: usize, height: usize) -> Vec<Edge> {
    let mut edges = Vec::new();
    for y in 0 .. height - 1 {
        for x in 0 .. width - 1 {
            edges.push((x, y, Direction::South));
            edges.push((x, y, Direction::East));
        }
    }
    edges
}