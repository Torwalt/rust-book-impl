use core::fmt;

use wasm_bindgen::prelude::*;

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
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbour_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                // "base" case.
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                // Apply the modulo in order to "wrap around" to the first col/row if at the edge.
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    // My initial impl. Is inefficient, because `if` is used to check for the edges.
    // The impl is wrong, because it does not implement the edge wrap around.
    fn live_neighbour_count2(&self, row: u32, col: u32) -> u8 {
        let pre_row = row.checked_sub(1);
        let next_row = (row < (self.height - 1)).then(|| row + 1);
        let pre_col = col.checked_sub(1);
        let next_col = (col < (self.width - 1)).then(|| col + 1);
        let mut count = 0;
        let neighbors: [(Option<u32>, Option<u32>); 8] = [
            (pre_row, pre_col),
            (pre_row, Some(col)),
            (pre_row, next_col),
            (Some(row), pre_col),
            (Some(row), next_col),
            (next_row, pre_col),
            (next_row, Some(col)),
            (next_row, next_col),
        ];

        for neighbor in neighbors {
            match neighbor {
                (Some(row), Some(col)) => {
                    let idx = self.get_index(row, col);
                    count += self.cells[idx] as u8;
                }
                // Out of bounds case.
                _ => continue,
            };
        }

        count
    }
}

#[wasm_bindgen]
impl Universe {
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

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbour_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
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
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const PERIODIC_PATTERN: [Cell; 25] = [
        //  0           1           2           3           4
        Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead, // 0
        Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead, Cell::Dead, // 1
        Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead, Cell::Dead, // 2
        Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead, Cell::Dead, // 3
        Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead, // 4
    ];

    #[test]
    fn live_neighbour_count() {
        let unv = Universe {
            width: 5,
            height: 5,
            cells: PERIODIC_PATTERN.to_vec(),
        };

        let count = unv.live_neighbour_count(2, 2);
        let exp_count = 2;
        assert_eq!(count, exp_count);

        let count = unv.live_neighbour_count(0, 0);
        let exp_count = 0;
        assert_eq!(count, exp_count);

        let count = unv.live_neighbour_count(4, 4);
        let exp_count = 0;
        assert_eq!(count, exp_count);
    }
}
