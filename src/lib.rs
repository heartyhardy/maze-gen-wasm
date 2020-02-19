mod utils;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cell {
    visited: bool,
    walls: u8,
}

#[wasm_bindgen]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Maze {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    stack: Vec<usize>,
    next: usize,
}

#[wasm_bindgen]
impl Maze {
    // Creates a new Maze with default settings
    pub fn new() -> Maze {
        let width: u32 = 5;
        let height: u32 = 5;
        let mut cells: Vec<Cell> = Vec::new();
        let stack: Vec<usize> = Vec::new();
        let next = 0;

        for _v in 0..width * height {
            let cell = Cell {
                visited: false,
                walls: 0,
            };
            cells.push(cell);
        }

        Maze {
            width,
            height,
            cells,
            stack,
            next,
        }
    }

    //Gets cells as ptr to u8 Array
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    //Maze width in cells
    pub fn width(&self) -> u32 {
        self.width
    }

    //Maze height in cells
    pub fn height(&self) -> u32 {
        self.height
    }

    //Gets the index of the linear array via given row and column
    pub fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    //Generate a maze
    pub fn gen_maze(&mut self) {
        if self.get_visited_count() < 25 {
            let next_result = self.get_unvisited(self.next);
            let has_next = next_result.is_ok();
            self.cells[self.next].visited = true;

            if has_next {
                let next = next_result.ok().unwrap();
                self.set_walls(self.next, next);
                self.next = next;
                self.stack.push(next);
            } else if !has_next && self.get_visited_count() < 25 {
                self.stack.pop();
                self.next = *self.stack.last().unwrap();
            }
        }
    }

    //gets an unvisited neighbor cell if exists
    fn get_unvisited(&self, index: usize) -> Result<usize, bool> {
        let mut rng = thread_rng();
        let row: i32 = (index / self.width as usize) as i32;
        let column = (index % self.width as usize) as i32;
        let mut neighbors = [
            (row - 1, column),
            (row, column + 1),
            (row + 1, column),
            (row, column - 1),
        ];
        neighbors.shuffle(&mut rng);

        for n in neighbors.iter() {
            if (n.0 >= 0 && n.0 < self.height as i32) && (n.1 >= 0 && n.1 < self.width as i32) {
                let idx = self.get_index(n.0 as u32, n.1 as u32);
                if !self.cells[idx].visited {
                    return Ok(idx as usize);
                }
            }
        }
        Err(false)
    }

    fn set_walls(&mut self, current: usize, next: usize) {
        if (current - self.width as usize) == next {
            self.cells[current].walls |= 1;
            self.cells[next].walls |= 4;
        } else if (current - 1) == next {
            self.cells[current].walls |= 8;
            self.cells[next].walls |= 2;
        } else if (current + 1) == next {
            self.cells[current].walls |= 2;
            self.cells[next].walls |=8;
        } else if (current + self.width as usize) == next {
            self.cells[current].walls |= 4;
            self.cells[next].walls |= 1;
        }
    }

    pub fn get_visited_count(&self) -> usize {
        self.cells
            .clone()
            .iter()
            .filter(|&e| e.visited == true)
            .collect::<Vec<&Cell>>()
            .len()
    }

    pub fn get_next_visited(&self) -> usize {
        self.next
    }

    pub fn has_next(&self) -> usize {
        let next = self.get_unvisited(self.next);
        if next.is_ok() {
            return next.ok().unwrap();
        }
        return 0;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &c in line {
                let symbol = if !c.visited { '◻' } else { '◼' };
                write!(f, "{0:>1}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
