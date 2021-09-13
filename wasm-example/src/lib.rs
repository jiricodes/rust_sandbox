mod utils;
// use std::fmt;

extern crate js_sys;
use js_sys::Math::random as rng;
use wasm_bindgen::prelude::*;
extern crate fixedbitset;
use fixedbitset::FixedBitSet;
extern crate web_sys;
use web_sys::console;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}

pub struct Timer<'a> {
	name: &'a str,
}

impl<'a> Timer<'a> {
	pub fn new(name: &'a str) -> Timer<'a> {
		console::time_with_label(name);
		Timer { name }
	}
}

impl<'a> Drop for Timer<'a> {
	fn drop(&mut self) {
		console::time_end_with_label(self.name);
	}
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
	cells: FixedBitSet,
}

impl Universe {
	fn get_index(&self, row: u32, column: u32) -> usize {
		(row * self.width + column) as usize
	}

	fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
		let mut count = 0;
		for delta_row in [self.height - 1, 0, 1].iter().cloned() {
			for delta_col in [self.width - 1, 0, 1].iter().cloned() {
				if delta_row == 0 && delta_col == 0 {
					continue;
				}

				let neighbor_row = (row + delta_row) % self.height;
				let neighbor_col = (column + delta_col) % self.width;
				let idx = self.get_index(neighbor_row, neighbor_col);
				count += self.cells[idx] as u8;
			}
		}
		count
	}

	fn set_cell_safe(&mut self, row: u32, column: u32, val: Cell) {
		let row = row % self.height;
		let column = column % self.width;
		let i = self.get_index(row, column);
		self.cells.set(i, val == Cell::Alive);
	}

	pub fn get_cells(&self) -> &FixedBitSet {
		&self.cells
	}

	pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
		for (row, col) in cells.iter().cloned() {
			self.set_cell_safe(row, col, Cell::Alive);
		}
	}
}

#[wasm_bindgen]
impl Universe {
	pub fn tick(&mut self) {
		let _timer = Timer::new("Universe::tick");
		let mut next = self.cells.clone();

		for row in 0..self.height {
			for col in 0..self.width {
				let idx = self.get_index(row, col);
				let cell = self.cells[idx];
				let live_neighbors = self.live_neighbor_count(row, col);

				next.set(
					idx,
					match (cell, live_neighbors) {
						(true, x) if x < 2 => false,
						(true, 2) | (true, 3) => true,
						(true, x) if x > 3 => false,
						(false, 3) => true,
						(otherwise, _) => otherwise,
					},
				);

				// if self.cells[idx] != next[idx] {
				// 	log!("[{}, {}] {} -> {}", row, col, self.cells[idx], next[idx])
				// }
			}
		}

		self.cells = next;
	}

	pub fn new() -> Universe {
		utils::set_panic_hook();
		let width = 64;
		let height = 64;

		let size = (width * height) as usize;
		let mut cells = FixedBitSet::with_capacity(size);

		for i in 0..size {
			cells.set(i, rng() < 0.5)
		}

		Universe {
			width,
			height,
			cells,
		}
	}

	pub fn random_grid(&mut self) {
		self.reset_grid();
		let size = (self.width * self.height) as usize;

		for i in 0..size {
			self.cells.set(i, rng() < 0.5)
		}
	}

	pub fn insert_glider(&mut self, row: u32, col: u32) {
		let cells = [
			(row + 1, col - 1),
			(row + 1, col),
			(row + 1, col + 1),
			(row, col + 1),
			(row - 1, col),
		];
		self.set_cells(&cells);
	}

	pub fn insert_pulsar(&mut self, row: u32, col: u32) {
		let cells = [
			// Center [6, 6]
			(row - 6, col - 4), // top left
			(row - 6, col - 3),
			(row - 6, col - 2),
			(row - 4, col - 6),
			(row - 4, col - 1),
			(row - 3, col - 6),
			(row - 3, col - 1),
			(row - 2, col - 6),
			(row - 2, col - 1),
			(row - 1, col - 4),
			(row - 1, col - 3),
			(row - 1, col - 2),
			(row - 6, col + 4), // top right
			(row - 6, col + 3),
			(row - 6, col + 2),
			(row - 4, col + 6),
			(row - 4, col + 1),
			(row - 3, col + 6),
			(row - 3, col + 1),
			(row - 2, col + 6),
			(row - 2, col + 1),
			(row - 1, col + 4),
			(row - 1, col + 3),
			(row - 1, col + 2),
			(row + 6, col - 4), // bottom left
			(row + 6, col - 3),
			(row + 6, col - 2),
			(row + 4, col - 6),
			(row + 4, col - 1),
			(row + 3, col - 6),
			(row + 3, col - 1),
			(row + 2, col - 6),
			(row + 2, col - 1),
			(row + 1, col - 4),
			(row + 1, col - 3),
			(row + 1, col - 2),
			(row + 6, col + 4), // bottom right
			(row + 6, col + 3),
			(row + 6, col + 2),
			(row + 4, col + 6),
			(row + 4, col + 1),
			(row + 3, col + 6),
			(row + 3, col + 1),
			(row + 2, col + 6),
			(row + 2, col + 1),
			(row + 1, col + 4),
			(row + 1, col + 3),
			(row + 1, col + 2),
		];
		self.set_cells(&cells);
	}

	// pub fn render(&self) -> String {
	// 	self.to_string()
	// }

	pub fn width(&self) -> u32 {
		self.width
	}

	pub fn height(&self) -> u32 {
		self.height
	}

	pub fn cells(&self) -> *const u32 {
		self.cells.as_slice().as_ptr()
	}

	pub fn set_width(&mut self, width: u32) {
		self.width = width;
		let size = (self.width * self.height) as usize;
		self.cells = FixedBitSet::with_capacity(size);
		self.reset_grid();
	}

	pub fn set_height(&mut self, height: u32) {
		self.height = height;
		let size = (self.width * self.height) as usize;
		self.cells = FixedBitSet::with_capacity(size);
		self.reset_grid();
	}

	pub fn toggle_cell(&mut self, row: u32, col: u32) {
		let i = self.get_index(row, col);
		self.cells.toggle(i);
	}

	pub fn reset_grid(&mut self) {
		self.cells.clear()
	}
}

// impl fmt::Display for Universe {
// 	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// 		for line in self.cells.as_slice().chunks(self.width as usize) {
// 			for &cell in line {
// 				let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
// 				write!(f, "{}", symbol)?;
// 			}
// 			write!(f, "\n")?;
// 		}

// 		Ok(())
// 	}
// }
