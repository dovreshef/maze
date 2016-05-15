use image::{self, DynamicImage, ImageBuffer};
use image::imageops::FilterType;
use std::path::Path;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::io;
use std::f32::EPSILON;
use grid::{Cell, Grid, Direction};

const BASE_STROKE_WIDTH: u32 = 3;
const BASE_CELL_LENGTH: u32 = 15;
const MARGIN_LENGTH: u32 = 15;
const BLACK: u8 = 0;
const WHITE: u8 = 255;

pub struct MazeRender {
    width: u32,
    height: u32,
    scale: f32,
    cells: Vec<Vec<Cell>>,
    img: Option<DynamicImage>
}

impl MazeRender {
    pub fn new(grid: &Grid) -> MazeRender {
        MazeRender {
            width: grid.width as u32,
            height: grid.height as u32,
            scale: 1.0,
            cells: grid.cells.clone(),
            img: None
        }
    }

    pub fn scale(&mut self, scale: f32) -> &mut MazeRender {
        self.scale = scale;
        self
    }

    pub fn render(&mut self) -> &mut MazeRender {
        let img_width = MARGIN_LENGTH * 2 + 
            self.width * (BASE_CELL_LENGTH + BASE_STROKE_WIDTH) + 
            BASE_STROKE_WIDTH;
        let img_height = MARGIN_LENGTH * 2 + 
            self.height * (BASE_CELL_LENGTH + BASE_STROKE_WIDTH) + 
            BASE_STROKE_WIDTH;
        let mut imgbuf = ImageBuffer::new(img_width, img_height);
        // Iterate over the coordiantes and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let mut value = image::Luma([WHITE]);
            let inside_margins = x >= MARGIN_LENGTH && x < (img_width - MARGIN_LENGTH) &&
                y >= MARGIN_LENGTH && y < (img_height - MARGIN_LENGTH);
            if inside_margins {
                let xt = x - MARGIN_LENGTH;
                let yt = y - MARGIN_LENGTH;
                let cell_len = BASE_CELL_LENGTH + BASE_STROKE_WIDTH;
                let x_in_stroke = xt % cell_len < BASE_STROKE_WIDTH;
                let y_in_stroke = yt % cell_len < BASE_STROKE_WIDTH;
                let mut x_dir = Direction::West;
                let mut y_dir = Direction::North;
                let mut x_idx = (xt / cell_len) as usize;
                let mut y_idx = (yt / cell_len) as usize;
                let mut border = false;
                // we treat the case where we hit the last wall of the maze
                if x_idx == self.width as usize {
                    x_idx -= 1;
                    x_dir = Direction::East;
                    border = true;
                }
                if y_idx == self.height as usize {
                    y_idx -= 1;
                    y_dir = Direction::South;
                    border = true;
                }
                // corner case :)
                let corner_closed = !border && x_in_stroke && y_in_stroke &&
                    ((x_idx > 0 && self.cells[x_idx - 1][y_idx].get(y_dir)) ||
                    (y_idx > 0 && self.cells[x_idx][y_idx - 1].get(x_dir)));                                
                let x_cell_closed = x_in_stroke && self.cells[x_idx][y_idx].get(x_dir);
                let y_cell_closed = y_in_stroke && self.cells[x_idx][y_idx].get(y_dir);
                if x_cell_closed || y_cell_closed || corner_closed {
                    value = image::Luma([BLACK]);
                }
            }
            *pixel = value;           
        }
        let mut img = DynamicImage::ImageLuma8(imgbuf);
        if (self.scale - 1.0).abs() > EPSILON {
            let new_width = (img_width as f32 * self.scale) as u32;
            let new_height = (img_height as f32 * self.scale) as u32;
            img = img.resize_exact(new_width, new_height, FilterType::Lanczos3);
        }
        self.img = Some(img);
        self
    }

    pub fn save_to_file(&self, path: &str) -> io::Result<()> {
        match self.img {
            Some(ref img) => {
                let mut path_str = String::from(path);
                if !path_str.ends_with(".png") {
                    path_str.push_str(".png");
                }
                let path = Path::new(&path_str);
                if path.exists() {
                    let err = Error::new(ErrorKind::Other, format!("Can't save to '{}'. Path already exists.", path_str));
                    return Err(err);
                } else {
                    let mut fout = try!(File::create(&path));
                    // We must indicate the image’s color type and what format to save as
                    let _ = img.save(&mut fout, image::PNG);
                }
            }
            None => {
                let err = Error::new(ErrorKind::Other, "The image has not been rendered yet.");
                return Err(err);
            }
        }
        Ok(())
    }
}
