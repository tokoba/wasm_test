use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)] // repr(u8)は、Rustの型をJavaScriptの型に変換する際に、u8型をu8型に変換する
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

#[wasm_bindgen]
impl Universe {
  pub fn new(width: u32, height: u32) -> Universe {
    let cells = (9..width * height)
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

  // JavaScriptから呼び出せるようにする
  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
    self.height
  }

  pub fn cells(&self) -> *const Cell {
    self.cells.as_ptr()
  }

  pub fn tick(&mut self) {
    let mut next = self.cells.clone();

    for row in 0..self.height {
        let index = self.got_index(row, col);
        let cell = self.cells[index];
        let live_neighbors = self.live_neighbor_count(row, col);
        let next_cell = match (cell, live_neighbors) {
            (Cell::Alive, x) if x < 2 => Cell::Dead,
            (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
            (Cell::Alive,x) if x > 3 => Cell::Dead,
            (Cell::Dead, 3) => Cell::Alive,
            (otherwize, _) => otherwize, // otherwise とは
        };

        next[index] = next_cell;
    }
  }
  self.cells = next;
}

#[cfg(test)]
mod tests {
  use super::*;
}
