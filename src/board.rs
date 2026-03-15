use crate::cell::{Cell, CellContent, HEIGHT, TOTAL_CELLS, WIDTH};

pub struct Board {
    pub grid: [Cell; TOTAL_CELLS],
}

impl Board {
    pub fn new() -> Self {
        Self {
            grid: [Cell::default(); TOTAL_CELLS],
        }
    }

    pub fn get_index(&self, x: usize, y: usize) -> Option<usize> {
        if x >= WIDTH || y >= HEIGHT {
            return None;
        }
        Some(y * WIDTH + x)
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        self.get_index(x, y).map(|i| &self.grid[i])
    }

    pub fn get_mut_cell(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        self.get_index(x, y).map(|i| &mut self.grid[i])
    }

    pub fn get_neighbor_coords(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::with_capacity(8);
        let min_x = x.saturating_sub(1);
        let max_x = (x + 1).min(WIDTH - 1);
        let min_y = y.saturating_sub(1);
        let max_y = (y + 1).min(HEIGHT - 1);

        for ny in min_y..=max_y {
            for nx in min_x..=max_x {
                if nx == x && ny == y {
                    continue;
                }
                neighbors.push((nx, ny));
            }
        }
        neighbors
    }

    pub fn count_neighbor_mines(&self, x: usize, y: usize) -> u8 {
        self.get_neighbor_coords(x, y)
            .iter()
            .filter_map(|(nx, ny)| self.get_cell(*nx, *ny))
            .filter(|cell| cell.content == CellContent::Mine)
            .count() as u8
    }
}
