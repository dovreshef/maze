#[derive(Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    fn opposite(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East  => Direction::West,
            Direction::West  => Direction::East
        }
    }
}

#[derive(Clone)]
pub struct Cell {
    north: bool,
    south: bool,
    east: bool,
    west: bool
}

impl Cell {
    fn new(closed: bool) -> Cell {
        Cell { 
            north: closed, 
            south: closed, 
            east: closed, 
            west: closed 
        }
    } 

    pub fn is_closed(&self) -> bool {
        self.north && self.south && self.east && self.west
    }

    fn set(&mut self, dir: Direction, closed: bool) {
        match dir {
            Direction::North => self.north = closed,
            Direction::South => self.south = closed,
            Direction::East => self.east = closed,
            Direction::West => self.west = closed
        }
    }

    pub fn get(&self, dir: Direction) -> bool {
        match dir {
            Direction::North => self.north,
            Direction::South => self.south,
            Direction::East => self.east,
            Direction::West => self.west
        }
    }    
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>
}

impl Grid {
    pub fn new(width: usize, height: usize, closed: bool) -> Grid {
        let cell = Cell::new(closed);
        Grid {
            width: width,
            height: height,
            cells: vec![vec![cell; height]; width]
        }
    }

    pub fn open(&mut self, x: usize, y: usize, dir: Direction) {
        self.set(x, y, dir, false);
    }

    pub fn close(&mut self, x: usize, y: usize, dir: Direction) {
        self.set(x, y, dir, true);
    }

    fn set(&mut self, x: usize, y: usize, dir: Direction, closed: bool) {
        self.cells[x][y].set(dir, closed);
        if let Some((nx, ny)) = self.cell_at(x, y, dir) {
            self.cells[nx][ny].set(dir.opposite(), closed);
        }
    }

    pub fn cell_at(&self, x: usize, y: usize, dir: Direction) -> Option<(usize, usize)> {
        match dir {
            Direction::North if y > 0                 => Some((x, y - 1)),
            Direction::South if y < self.height - 1   => Some((x, y + 1)),
            Direction::East  if x < self.width - 1    => Some((x + 1, y)),
            Direction::West  if x > 0                 => Some((x - 1, y)),
            _                                         => None           
        }
    }   
}
