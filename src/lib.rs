mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        [(-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1)].iter()
        .map(|(row_offset, col_offset)| 
            (((row as i32 + row_offset) as u32).rem_euclid(self.height), 
            ((column as i32 + col_offset) as u32).rem_euclid(self.width)))
        .map(|(neighbour_row, neighbour_column)| self.get_index(neighbour_row, neighbour_column))
        .map(|index| self.cells[index])
        .filter(|cell| cell == &Cell::Alive)
        .count() as u8
    }
}
 
#[wasm_bindgen]
impl Universe {
    fn cell_at_position(&self, row: u32, column: u32) -> Cell {
        self.cells[self.get_index(row, column)]
    }

    pub fn tick(&mut self) {
        self.cells = (0..self.height)
            .flat_map(|row| (0..self.width)
                .map(move |col| (row, col)))
            .map(|(row, col)| {
                let neigh_count = self.live_neighbor_count(row, col);
                match self.cell_at_position(row, col) {
                    Cell::Alive => {
                        if neigh_count > 1 && neigh_count < 4 {
                            Cell::Alive
                        } else {
                            Cell::Dead
                        }
                    },
                    Cell::Dead => {
                        if neigh_count == 3 {
                            Cell::Alive
                        } else {
                            Cell::Dead
                        }
                    }
                }})
            .collect()
    }

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
        .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        self.cells.as_slice()
            .chunks(self.width as usize)
            .into_iter()
            .flat_map(|cells|
                cells.into_iter()
                .map(|cell| match cell {
                    Cell::Alive => '🤠',
                    Cell::Dead => '💀' })
                .chain(Some('\n')))
            .for_each(|cells: char| write!(f, "{}", cells).unwrap());
        Ok(())
    }
}