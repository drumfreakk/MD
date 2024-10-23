
/*! Some helper functions for displaying the graph.

Original code from <https://github.com/plotters-rs/plotters-minifb-demo>
The other code for displaying the graph in main.rs is also adapted from this repo
*/

use std::borrow::{Borrow, BorrowMut};
use embedded_graphics_core::pixelcolor::{Rgb888, IntoStorage};
use embedded_graphics_core::prelude::Point;

pub struct FrameBuffer {
	pub buffer: Vec<u32>,
	pub width: usize,
	pub height: usize,
}

impl Borrow<[u8]> for FrameBuffer {
	fn borrow(&self) -> &[u8] {
		// Safe for alignment: align_of(u8) <= align_of(u32)
		// Safe for cast: u32 can be thought of as being transparent over [u8; 4]
		unsafe {
			std::slice::from_raw_parts(
				self.buffer.as_ptr() as *const u8,
				self.buffer.len() * 4
			)
		}
	}
}

impl Borrow<[u32]> for FrameBuffer {
	fn borrow(&self) -> &[u32] {
		self.buffer.as_slice()
	}
}

impl FrameBuffer {
	pub fn new(width: usize, height: usize) -> Self {
		FrameBuffer {
			buffer: vec![0u32; width * height],
			width,
			height,
		}
	}

	pub fn fill_buffer(&mut self, color: Rgb888) {
		for i in 0..(self.width * self.height) {
			self.buffer[i] = color.into_storage() as u32;
		}
	}

	pub fn clear_buffer(&mut self) {
		for i in 0..(self.width * self.height) {
			self.buffer[i] = 0;
		}
	}

	#[inline]
	pub fn draw_point(&mut self, x: i32, y: i32, c: Rgb888) {
		if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
			self.buffer[y as usize * self.width + x as usize] = c.into_storage() as u32;
		}
	}

	pub fn draw_horizontal_line(&mut self, p1: [i32; 2], p2: [i32; 2], color: Rgb888){
		let start = p1[0].min(p2[0]);
		let end = p1[0].max(p2[0]);
	
		for x in start..=end {
			self.draw_point(x, p1[1], color);
		}
	}
}
