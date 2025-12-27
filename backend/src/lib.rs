use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)] // repr(u8)は、Rustの型をJavaScriptの型に変換する際に、u8型をu8型に変換する
#[derive(Clone, Copy)]
pub enum Cell {
  Dead = 0,
  Alive = 1,
}

#[wasm_bindgen]
#[derive(Clone)]
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
      for col in 0..self.width {
        let index = self.get_index(row, col);
        let cell = self.cells[index];
        let live_neighbors = self.live_neighbor_count(row, col);
        let next_cell = match (cell, live_neighbors) {
          (Cell::Alive, x) if x < 2 => Cell::Dead,
          (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
          (Cell::Alive, x) if x > 3 => Cell::Dead,
          (Cell::Dead, 3) => Cell::Alive,
          (otherwize, _) => otherwize,
        };

        next[index] = next_cell;
      }
    }
    self.cells = next;
  }

  fn get_index(&self, row: u32, col: u32) -> usize {
    (row * self.width + col) as usize
  }

  fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
    let mut count = 0;
    for delta_row in [self.height - 1, 0, 1].iter().cloned() {
      for delta_col in [self.width - 1, 0, 1].iter().cloned() {
        if delta_row == 0 && delta_col == 0 {
          continue;
        }
        let neighbor_row = (row + delta_row) % self.height;
        let neighbor_col = (col + delta_col) % self.width;
        let index = self.get_index(neighbor_row, neighbor_col);
        count += self.cells[index] as u8;
      }
    }
    count
  }
}

#[cfg(test)]
mod tests {
  // use super::*;
}
