use crate::cell::{Cell, CellContent, HEIGHT, TOTAL_CELLS, WIDTH};
#[derive(Clone, Copy)]
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
        if y >= HEIGHT || x >= WIDTH {
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
        let mut neighbors = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let ny = y as i32 + dy;
                let nx = x as i32 + dx;
                if nx >= 0 && nx < WIDTH as i32 && ny >= 0 && ny < HEIGHT as i32 {
                    neighbors.push((nx as usize, ny as usize));
                }
            }
        }
        neighbors
    }
    pub fn count_neighbor_mines(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for (x, y) in self.get_neighbor_coords(x, y) {
            if let Some(cell) = self.get_cell(x, y) {
                if cell.content == CellContent::Mine {
                    count += 1;
                }
            }
        }
        count
    }
}
